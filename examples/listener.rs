use std::time::Duration;

use futures::AsyncReadExt;
use tokio::time::sleep;

use srt_rs;
use srt_rs::statistics::Statistics;

#[tokio::main]
async fn main() {
    let mut connect = srt_rs::async_builder()
        .set_live_transmission_type()
        .set_peer_latency(1000)
        .listen("127.0.0.1:5555", 1)
        .unwrap();

    for (mut stream, sock_id) in connect.accept().await {
        println!("client ip: {}", sock_id);

        let mut statistics = Statistics::new(connect.socket.id);
        let statistics_task = tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                if let Err(e) = statistics.set() {
                    println!("{}", e);
                };
                println!("received packets: {}", statistics.statistics.pktRecvTotal);
            }
        });

        let mut buf = [0; 1316];
        loop {
            if let Err(e) = stream.read(&mut buf).await {
                statistics_task.abort();
                println!("cannot read from the stream: {}", e);
            }
        }
    }
}
