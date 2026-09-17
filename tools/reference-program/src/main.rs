use clap::{Parser, Subcommand};
use rm_recruit_reference::{Frame, read_frame, serve};
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

#[derive(Parser)]
#[command(version, about = "RM 通用协议参考程序")]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}
#[derive(Subcommand)]
enum Mode {
    Server {
        #[arg(long, default_value = "127.0.0.1:7878")]
        bind: String,
        #[arg(long, default_value_t = 30_000)]
        read_timeout_ms: u64,
        #[arg(long, default_value_t = 2_000)]
        shutdown_grace_ms: u64,
    },
    Client {
        #[arg(long, default_value = "127.0.0.1:7878")]
        address: String,
        #[arg(long, default_value_t = 12_000)]
        response_timeout_ms: u64,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Args::parse().mode {
        Mode::Server {
            bind,
            read_timeout_ms,
            shutdown_grace_ms,
        } => {
            let address: std::net::SocketAddr = bind.parse()?;
            if !address.ip().is_loopback() {
                return Err("only loopback addresses are supported".into());
            }
            let listener = TcpListener::bind(address).await?;
            println!("LISTENING {}", listener.local_addr()?);
            serve(listener, read_timeout_ms, shutdown_grace_ms, async {
                let _ = tokio::signal::ctrl_c().await;
            })
            .await?;
        }
        Mode::Client {
            address,
            response_timeout_ms,
        } => {
            let stream = TcpStream::connect(address).await?;
            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);
            for line in io::stdin().lock().lines() {
                let line = line?;
                if line.len() > 65536 {
                    return Err("request exceeds 65536 bytes".into());
                }
                writer.write_all(format!("{line}\n").as_bytes()).await?;
                match timeout(
                    Duration::from_millis(response_timeout_ms),
                    read_frame(&mut reader),
                )
                .await??
                {
                    Frame::Message(raw) => println!("{}", String::from_utf8(raw)?),
                    _ => return Err("peer closed or returned an invalid frame".into()),
                }
            }
        }
    }
    Ok(())
}
