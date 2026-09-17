use rm_recruit_reference::{Frame, Store, dispatch, read_frame, serve};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, oneshot};
use tokio::time::timeout;

async fn response(stream: &mut BufReader<TcpStream>) -> Value {
    match timeout(Duration::from_secs(3), read_frame(stream))
        .await
        .unwrap()
        .unwrap()
    {
        Frame::Message(raw) => serde_json::from_slice(&raw).unwrap(),
        _ => panic!("expected response"),
    }
}
async fn start(
    read_ms: u64,
    grace_ms: u64,
) -> (
    std::net::SocketAddr,
    oneshot::Sender<()>,
    tokio::task::JoinHandle<std::io::Result<()>>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, receiver) = oneshot::channel();
    let task = tokio::spawn(serve(listener, read_ms, grace_ms, async {
        let _ = receiver.await;
    }));
    (address, stop, task)
}
#[tokio::test]
async fn all_actions_and_validation() {
    let state: Store = Arc::new(Mutex::new(BTreeMap::new()));
    let cases = [
        (json!({"id":1,"action":"ping"}), json!("pong")),
        (json!({"id":2,"action":"echo","data":"你好"}), json!("你好")),
        (
            json!({"id":3,"action":"delay","milliseconds":0,"data":"ok"}),
            json!("ok"),
        ),
        (
            json!({"id":4,"action":"text_stats","text":"a\n🙂"}),
            json!({"characters":3,"lines":2}),
        ),
        (
            json!({"id":5,"action":"number_stats","numbers":[-2,1,4]}),
            json!({"count":3,"min":-2.0,"max":4.0,"mean":1.0}),
        ),
        (
            json!({"id":6,"action":"set","key":"a","value":"b"}),
            json!({}),
        ),
        (
            json!({"id":7,"action":"get","key":"a"}),
            json!({"value":"b"}),
        ),
        (json!({"id":8,"action":"list"}), json!(["a"])),
        (json!({"id":9,"action":"delete","key":"a"}), json!({})),
    ];
    for (request, data) in cases {
        let actual = dispatch(request.to_string().as_bytes(), &state).await;
        assert_eq!(actual["ok"], true, "{actual}");
        assert_eq!(actual["data"], data);
    }
    for request in [
        json!({"id":true,"action":"ping"}),
        json!({"id":1,"action":"delay","milliseconds":-1,"data":""}),
        json!({"id":1,"action":"number_stats","numbers":[true]}),
        json!({"id":1,"action":"number_stats","numbers":[]}),
        json!({"id":1,"action":"ping","extra":1}),
    ] {
        assert_eq!(
            dispatch(request.to_string().as_bytes(), &state).await["error"]["code"],
            "invalid_request"
        );
    }
    assert_eq!(
        dispatch(b"{", &state).await["error"]["code"],
        "invalid_json"
    );
    assert_eq!(
        dispatch(br#"{"id":1,"action":"get","key":"a"}"#, &state).await["error"]["code"],
        "not_found"
    );
}

#[tokio::test]
async fn concurrent_connections_and_error_recovery() {
    let (address, stop, task) = start(3000, 100).await;
    let mut a = BufReader::new(TcpStream::connect(address).await.unwrap());
    a.get_mut()
        .write_all(b"{\"id\":1,\"action\":\"delay\",\"milliseconds\":1200,\"data\":\"a\"}\n")
        .await
        .unwrap();
    assert!(
        timeout(Duration::from_millis(80), read_frame(&mut a))
            .await
            .is_err()
    );
    let mut b = BufReader::new(TcpStream::connect(address).await.unwrap());
    b.get_mut()
        .write_all(b"{\n{\"id\":2,\"action\":\"ping\"}\n")
        .await
        .unwrap();
    assert_eq!(response(&mut b).await["error"]["code"], "invalid_json");
    let pong = timeout(Duration::from_millis(500), response(&mut b))
        .await
        .unwrap();
    assert_eq!(pong["data"], "pong");
    assert_eq!(response(&mut a).await["data"], "a");
    stop.send(()).unwrap();
    timeout(Duration::from_secs(1), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn read_timeout_and_oversized_frame_are_isolated() {
    let (address, stop, task) = start(100, 100).await;
    let mut idle = BufReader::new(TcpStream::connect(address).await.unwrap());
    idle.get_mut().write_all(b"{").await.unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(1), read_frame(&mut idle))
            .await
            .unwrap()
            .unwrap(),
        Frame::Eof
    ));
    let mut large = BufReader::new(TcpStream::connect(address).await.unwrap());
    large.get_mut().write_all(&vec![b'x'; 65539]).await.unwrap();
    assert_eq!(response(&mut large).await["error"]["code"], "line_too_long");
    let mut normal = BufReader::new(TcpStream::connect(address).await.unwrap());
    normal
        .get_mut()
        .write_all(b"{\"id\":1,\"action\":\"ping\"}\n")
        .await
        .unwrap();
    assert_eq!(response(&mut normal).await["ok"], true);
    stop.send(()).unwrap();
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn graceful_shutdown_finishes_inflight_and_closes_idle() {
    let (address, stop, task) = start(5000, 1500).await;
    let mut active = BufReader::new(TcpStream::connect(address).await.unwrap());
    let mut idle = BufReader::new(TcpStream::connect(address).await.unwrap());
    active
        .get_mut()
        .write_all(b"{\"id\":1,\"action\":\"delay\",\"milliseconds\":400,\"data\":\"done\"}\n")
        .await
        .unwrap();
    assert!(
        timeout(Duration::from_millis(80), read_frame(&mut active))
            .await
            .is_err()
    );
    stop.send(()).unwrap();
    assert!(matches!(
        timeout(Duration::from_millis(250), read_frame(&mut idle))
            .await
            .unwrap()
            .unwrap(),
        Frame::Eof
    ));
    assert_eq!(response(&mut active).await["data"], "done");
    timeout(Duration::from_secs(2), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn shutdown_aborts_requests_over_grace() {
    let (address, stop, task) = start(5000, 50).await;
    let mut active = BufReader::new(TcpStream::connect(address).await.unwrap());
    active
        .get_mut()
        .write_all(b"{\"id\":1,\"action\":\"delay\",\"milliseconds\":10000,\"data\":\"late\"}\n")
        .await
        .unwrap();
    assert!(
        timeout(Duration::from_millis(80), read_frame(&mut active))
            .await
            .is_err()
    );
    stop.send(()).unwrap();
    timeout(Duration::from_secs(1), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(matches!(read_frame(&mut active).await.unwrap(), Frame::Eof));
}
