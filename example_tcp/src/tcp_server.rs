use crate::commands::Command;
use std::io;
use std::net::SocketAddr;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufStream, BufWriter};
use tokio::net::{TcpListener, TcpStream};

pub async fn start_server(addr: &str) -> io::Result<()> {
    let addr = addr.parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket).await.unwrap();
        });
    }
}

async fn handle_client(socket: TcpStream) -> io::Result<()> {
    let (reader, writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    loop {
        println!("thread={:?}",std::thread::current());
        let mut buffer = String::new();
        reader.read_line( &mut buffer).await.unwrap();
        println!("1111={:?}",buffer);

        if buffer.is_empty() {
            break;
        }

        let command: Command = match serde_json::from_str(&buffer) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("err={}",e);
                Command::CmdError { message: e.to_string() }
                // break
            },
        };

        println!("cmd={:?}",command);

        let response = match command {
            Command::GetHelloWorld => "Hello, world!".to_string(),
            Command::PostGreeting { message } => format!("{}", message),
            Command::CmdError { message } => format!("{}", message),
        };

        // let response = json!({"PostGreeting": {"message": "Ok,PostGreeting!Too"}}).to_string() + "\n";
        let v = response.to_string()+ "\n"+",test1test2"+"\n";
        println!("v={}",v);
        println!("write_all={:?}",writer.write_all(v.as_bytes()).await?);
        println!("is_empty1={:?}",writer.buffer().is_empty());
        println!("flush={:?}",writer.flush().await?);
        println!("is_empty2={:?}",writer.buffer().is_empty());
        // break;
    }

    println!("?????loop?????={:?}",std::time::SystemTime::now());

    Ok(())
}