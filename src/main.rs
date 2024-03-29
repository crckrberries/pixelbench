use rand::{self, Rng};
use std::error::Error;
use std::fmt::Debug;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:1234";

    let ln: TcpListener = TcpListener::bind(&addr).await?;

    loop {
        let (mut socket, _) = ln.accept().await.expect("socket couldnt connect");

        tokio::spawn(async move {
            let mut start = Instant::now();
            let mut bytes: f32 = 0.0; // bytes per second
            let mut totalbytes = 0.0; // bytes sent total

            loop {
                let mut buf = vec![0; 4096];
                let n = socket.read(&mut buf).await.unwrap();

                bytes += n as f32;
                totalbytes += n as f32;

                buf.truncate(n);
                let str = String::from_utf8(buf).unwrap();
                match str.as_str() {
                    "SIZE\n" => {
                        socket
                            .write_all("SIZE 1280 720\n".as_bytes())
                            .await
                            .unwrap();
                    }
                    "HELP\n" => {
                        socket.write_all("this isnt pixelflut lmao\n".as_bytes()).await.unwrap();
                    }
                    _ => {
                        if str.starts_with("PX") && str.split(" ").count() == 3 {
                            let mut ss = str.split(" ");
                            let (x, y): (u32, u32) = (
                                ss.nth(1).unwrap().parse().unwrap(),
                                ss.next().unwrap()[..2].parse().unwrap(),
                            );
                            socket
                                .write_all(
                                    format!(
                                        "PX {x} {y} {:02x}{:02x}{:02x}{:02x}\n",
                                        rand::thread_rng().gen_range(0..255),
                                        rand::thread_rng().gen_range(0..255),
                                        rand::thread_rng().gen_range(0..255),
                                        rand::thread_rng().gen_range(0..255)
                                    )
                                    .as_bytes(),
                                )
                                .await
                                .unwrap();
                        };
                    }
                }

                if n == 0 {
                    // if the connection is over, display the time it took to send n bytes
                    println!(
                        "took {}ms to send {} megabytes",
                        start.elapsed().as_millis(),
                        totalbytes / 1000000.0
                    );
                    return;
                }

                let elapsed = start.elapsed(); // if the connection is not over by 1s, display megabits per second
                if elapsed.as_millis() >= 1000 {
                    println!("{} MBps", bytes / 1000000.0);
                    bytes = 0.0;
                    start = Instant::now();
                }
            }
        });
    }
}
