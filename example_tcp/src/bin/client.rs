
use serde_json::json;
use tokio::{io::{ AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}, net::TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conn = TcpStream::connect("127.0.0.1:8080").await?;
    let ( read_half,  write_half) = conn.into_split();

    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    // Send PostGreeting command
    // let post_greeting = json!({"GetHelloWorld": {"message": "GetHelloWorld"}}).to_string() + "\n";
    // writer.write_all(post_greeting.as_bytes()).await?;
    // writer.flush().await?;


    // Send PostGreeting command
    let post_greeting = json!({"PostGreeting": {"message": "Ok,PostGreeting!"}}).to_string() + "\n";
    writer.write_all(post_greeting.as_bytes()).await?;
    writer.flush().await?;


    let mut response = Vec::new();
    loop{
        let mut vec = Vec::with_capacity(12);
        println!("read_buf len={:?}",reader.read_buf(&mut vec).await?);
        if reader.buffer().is_empty(){
            response.extend(vec.iter().copied());
            break
        }
        
        response.extend(vec.iter().copied());
    }

    let response = String::from_utf8(response).expect("Our bytes should be valid utf8");
    println!("5555={:?},len={},line={}",response,response.len(),response.lines().count());

    Ok(())
}