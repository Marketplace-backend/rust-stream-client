use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut connection = tokio::net::TcpStream::connect("localhost:8080").await?;

        let mut bytes = vec![];
        let mut file = File::create("./example/video.mp4")?;
    loop {
        let byte_video = tokio::io::copy(&mut connection, &mut bytes).await?/*connection.read_buf(&mut bytes).await?*/;
        if byte_video == 0 {
            break;
        }
        println!("{}", byte_video);
        file.write(&mut bytes)?;
        thread::sleep(Duration::from_secs(2));
    }
    let video = tokio::fs::read("./example/video.mp4").await?;
    println!("Video size: {}", video.len());
    Ok(())
}