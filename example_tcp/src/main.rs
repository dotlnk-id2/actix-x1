mod commands;
mod tcp_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let rt = tokio::runtime::Builder::new_multi_thread()
    // .worker_threads(8)  // 8个工作线程
    // .enable_io()        // 可在runtime中使用异步IO
    // .enable_time()      // 可在runtime中使用异步计时器(timer)
    // .build()            // 创建runtime
    // .unwrap();

    // rt.spawn(async{
        tcp_server::start_server("127.0.0.1:8080").await
    // });

    // rt.block_on(async{
    //     tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    // });

    // Ok(())
}