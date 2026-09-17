use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, watch};
use tokio::task::JoinSet;
use tokio::time::timeout;

pub const MAX_LINE: usize = 65_536;
pub const MAX_ID: u64 = 9_007_199_254_740_991;
pub type Store = Arc<Mutex<BTreeMap<String, String>>>;

pub fn error(id: Value, code: &str, message: &str) -> Value {
    json!({"id": id, "ok": false, "error": {"code": code, "message": message}})
}
fn success(id: u64, data: Value) -> Value {
    json!({"id": id, "ok": true, "data": data})
}

pub async fn dispatch(raw: &[u8], store: &Store) -> Value {
    let request: Value = match serde_json::from_slice(raw) {
        Ok(value) => value,
        Err(_) => return error(Value::Null, "invalid_json", "expected UTF-8 JSON"),
    };
    let Some(object) = request.as_object() else {
        return error(Value::Null, "invalid_request", "expected object");
    };
    let Some(id) = object
        .get("id")
        .and_then(Value::as_u64)
        .filter(|id| *id <= MAX_ID)
    else {
        return error(Value::Null, "invalid_request", "invalid id");
    };
    let bad = |message: &str| error(json!(id), "invalid_request", message);
    let Some(action) = object.get("action").and_then(Value::as_str) else {
        return bad("action must be a string");
    };
    let fields: &[&str] = match action {
        "ping" | "list" => &["id", "action"],
        "echo" => &["id", "action", "data"],
        "delay" => &["id", "action", "data", "milliseconds"],
        "text_stats" => &["id", "action", "text"],
        "number_stats" => &["id", "action", "numbers"],
        "set" => &["id", "action", "key", "value"],
        "get" | "delete" => &["id", "action", "key"],
        _ => return error(json!(id), "unknown_action", "action is not supported"),
    };
    if object.len() != fields.len() || fields.iter().any(|key| !object.contains_key(*key)) {
        return bad("missing or extra fields");
    }
    match action {
        "ping" => success(id, json!("pong")),
        "echo" | "delay" => {
            let Some(data) = request["data"].as_str() else {
                return bad("data must be a string");
            };
            if action == "delay" {
                let Some(ms) = request["milliseconds"].as_u64().filter(|ms| *ms <= 10_000) else {
                    return bad("milliseconds must be an integer between 0 and 10000");
                };
                tokio::time::sleep(Duration::from_millis(ms)).await;
            }
            success(id, json!(data))
        }
        "text_stats" => {
            let Some(text) = request["text"].as_str() else {
                return bad("text must be a string");
            };
            success(
                id,
                json!({"characters": text.chars().count(),
                "lines": if text.is_empty() {0} else {text.matches('\n').count()+1}}),
            )
        }
        "number_stats" => {
            let Some(values) = request["numbers"].as_array() else {
                return bad("numbers must be a list");
            };
            if values.is_empty() || values.len() > 10_000 {
                return bad("list length must be 1..10000");
            }
            let numbers: Option<Vec<f64>> = values
                .iter()
                .map(|v| v.as_f64().filter(|n| n.is_finite() && n.abs() <= 1e100))
                .collect();
            let Some(numbers) = numbers else {
                return bad("expected finite numbers with abs <= 1e100");
            };
            let count = numbers.len();
            let min = numbers.iter().copied().fold(f64::INFINITY, f64::min);
            let max = numbers.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let mean = numbers.iter().map(|n| n / count as f64).sum::<f64>();
            success(
                id,
                json!({"count": count, "min": min, "max": max, "mean": mean}),
            )
        }
        "list" => {
            let state = store.lock().await;
            success(id, json!(state.keys().collect::<Vec<_>>()))
        }
        "set" | "get" | "delete" => {
            let Some(key) = request["key"]
                .as_str()
                .filter(|k| !k.is_empty() && k.chars().count() <= 64)
            else {
                return bad("key must contain 1..64 characters");
            };
            let mut state = store.lock().await;
            match action {
                "set" => {
                    let Some(value) = request["value"]
                        .as_str()
                        .filter(|v| v.chars().count() <= 4096)
                    else {
                        return bad("value must be a string of at most 4096 characters");
                    };
                    if !state.contains_key(key) && state.len() >= 128 {
                        return error(json!(id), "store_full", "at most 128 keys");
                    }
                    state.insert(key.to_owned(), value.to_owned());
                    success(id, json!({}))
                }
                "get" => match state.get(key) {
                    Some(value) => success(id, json!({"value": value})),
                    None => error(json!(id), "not_found", "key does not exist"),
                },
                _ => match state.remove(key) {
                    Some(_) => success(id, json!({})),
                    None => error(json!(id), "not_found", "key does not exist"),
                },
            }
        }
        _ => unreachable!(),
    }
}

pub enum Frame {
    Message(Vec<u8>),
    Eof,
    TooLong,
    Incomplete,
}
pub async fn read_frame<R: AsyncBufRead + Unpin>(reader: &mut R) -> io::Result<Frame> {
    let mut output = Vec::new();
    loop {
        let bytes = reader.fill_buf().await?;
        if bytes.is_empty() {
            return Ok(if output.is_empty() {
                Frame::Eof
            } else {
                Frame::Incomplete
            });
        }
        let end = bytes.iter().position(|byte| *byte == b'\n');
        let consumed = end.map_or(bytes.len(), |index| index + 1);
        if output.len() + consumed > MAX_LINE + 2 {
            return Ok(Frame::TooLong);
        }
        output.extend_from_slice(&bytes[..consumed]);
        reader.consume(consumed);
        if end.is_some() {
            output.pop();
            if output.last() == Some(&b'\r') {
                output.pop();
            }
            return Ok(if output.len() > MAX_LINE {
                Frame::TooLong
            } else {
                Frame::Message(output)
            });
        }
    }
}
async fn send<W: tokio::io::AsyncWrite + Unpin>(writer: &mut W, value: Value) -> io::Result<()> {
    let mut data = serde_json::to_vec(&value)?;
    if data.len() > MAX_LINE {
        data = serde_json::to_vec(&error(
            value["id"].clone(),
            "response_too_large",
            "response exceeds limit",
        ))?;
    }
    data.push(b'\n');
    timeout(Duration::from_secs(12), writer.write_all(&data))
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "write timeout"))?
}
async fn connection(
    stream: TcpStream,
    store: Store,
    read_ms: u64,
    mut stopping: watch::Receiver<bool>,
) -> io::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    loop {
        if *stopping.borrow() {
            return Ok(());
        }
        let frame = tokio::select! {
            biased;
            _ = stopping.changed() => return Ok(()),
            result = timeout(Duration::from_millis(read_ms), read_frame(&mut reader)) => {
                match result {
                    Ok(frame) => frame?,
                    Err(_) => return Ok(()),
                }
            }
        };
        match frame {
            Frame::Eof | Frame::Incomplete => return Ok(()),
            Frame::TooLong => {
                send(
                    &mut writer,
                    error(Value::Null, "line_too_long", "frame exceeds 65536 bytes"),
                )
                .await?;
                return Ok(());
            }
            Frame::Message(raw) => send(&mut writer, dispatch(&raw, &store).await).await?,
        }
    }
}

pub async fn serve<F: std::future::Future<Output = ()>>(
    listener: TcpListener,
    read_ms: u64,
    grace_ms: u64,
    shutdown: F,
) -> io::Result<()> {
    let store: Store = Arc::new(Mutex::new(BTreeMap::new()));
    let (stop, stopping) = watch::channel(false);
    let mut tasks = JoinSet::new();
    tokio::pin!(shutdown);
    loop {
        tokio::select! {
            biased;
            _ = &mut shutdown => break,
            result = tasks.join_next(), if !tasks.is_empty() => {
                if let Some(Ok(Err(error))) = result { eprintln!("connection ended: {error}"); }
            }
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                tasks.spawn(connection(stream, store.clone(), read_ms, stopping.clone()));
            }
        }
    }
    drop(listener);
    let _ = stop.send(true);
    if timeout(Duration::from_millis(grace_ms), async {
        while tasks.join_next().await.is_some() {}
    })
    .await
    .is_err()
    {
        tasks.shutdown().await;
    }
    Ok(())
}
