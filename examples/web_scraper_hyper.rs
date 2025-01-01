use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;

use hyper::Uri;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use std::{fs, thread};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let count = 10_000;
    let url = "http://jsonplaceholder.typicode.com/todos/2";
    let path = "/tmp/a";

    if fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false) {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir(path)?;

    // Create an HTTP client
    let client = Arc::new(Client::new());

    // Measure the total time
    let start_time = Instant::now();

    // Parse our URL...
    let url = url.parse::<Uri>()?;

    // Get the host and the port
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);

    // Create the Hyper client
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // Spawn a task to poll the connection, driving the HTTP state
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }

        // Create an HTTP request with an empty body and a HOST header
        let req = Request::builder()
            .uri(url)
            .body(Empty::<Bytes>::new())
            .unwrap();

        // Await the response...
        let mut res = sender.send_request(req).await.unwrap();

        println!("Response status: {}", res.status());

        // Stream the body, writing each frame to stdout as it arrives
        while let Some(next) = res.frame().await {
            let frame = next.unwrap();
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await.unwrap();
            }
        }
    });

    thread::park();

    // // Wait for all tasks to complete
    // println!("Waiting for tasks to complete...");
    // for (_i, handle) in tasks {
    //     handle.await??;
    // }
    println!("All tasks completed!");

    // Print the total time
    let duration = start_time.elapsed();
    println!("Total time: {:?}", duration);
    println!("{:?} req/s", count as f32 / duration.as_secs_f32());

    println!("Cleaning up...");
    // fs::remove_dir_all(path)?;

    Ok(())
}
