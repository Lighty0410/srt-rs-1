use futures::AsyncReadExt;
use std::time::Duration;
use tokio::time::sleep;

use srt_rs;
use srt_rs::statistics::Statistics;

#[tokio::main]
async fn main() {
    let mut connect = srt_rs::async_builder()
        .set_live_transmission_type()
        .set_receive_latency(1000)
        .connect("0.0.0.0:0")
        .unwrap()
        .await
        .unwrap();

    let mut statistics = Statistics::new(connect.socket.id);
    let statistics_task = tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            statistics.set().unwrap();
        }
    });

    let mut buf = [0; 1316];
    loop {
        if let Err(e) = connect.read(&mut buf).await {
            statistics_task.abort();
        };
    }
}
