use std::error::Error;
use std::time::Instant;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:1234";

    let ln: TcpListener = TcpListener::bind(&addr).await?;

    loop {
        let (mut socket, _) = ln.accept().await.expect("socket couldnt connect");

        tokio::spawn(async move {
            let mut start = Instant::now();
            let mut buf = vec![0; 4096];
            let mut bytes: f32 = 0.0; // bytes per second
            let mut totalbytes = 0.0; // bytes sent total

            loop {
                let n = socket.read(&mut buf).await.unwrap();
                // buf.truncate(n);

                bytes += n as f32;
                totalbytes += n as f32;

                if n == 0 {
                    // if the connection is over, display the time it took to send n bytes
                    println!(
                        "took {}ms to send {} megabytes",
                        start.elapsed().as_millis(),
                        totalbytes / 1000.0
                    );
                    totalbytes = 0.0;
                    return;
                }

                let elapsed = start.elapsed(); // if the connection is not over by 1s, display megabits per second
                if elapsed.as_millis() >= 1000 {
                    println!("{} Mbps", bytes / 1000.0);
                    bytes = 0.0;
                    start = Instant::now();
                }
            }
        });
    }
}
