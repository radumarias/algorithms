use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::sync::Arc;
use std::time::Instant;

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

    // Build a Reqwest client with certificate validation disabled.
    // This is insecure in production!
    let client = Arc::new(
        Client::builder()
            // .danger_accept_invalid_certs(true) // (Optional) Disable certificate validation
            // .danger_accept_invalid_hostnames(true) // (Optional) Disable hostname verification
            // .redirect(reqwest::redirect::Policy::none()) // Disable automatic redirection
            .build()?,
    );

    // Measure the total time
    let start_time = Instant::now();

    // Spawn concurrent tasks
    let mut tasks = Vec::with_capacity(count);
    for i in 1..=count {
        let client2 = client.clone();
        let handle = tokio::spawn(async move {
            let response = client2.get(url).send().await?;
            // let response = reqwest::get(url).await?;
            let todo: Todo = response.json().await?;
            // println!("{todo:?}");

            // Convert the struct to a pretty JSON string
            let json_string = serde_json::to_string_pretty(&todo).unwrap();
            // Save each response to /tmp/todo-#.json
            let path = format!("{path}/todo-{i}.json");
            fs::write(&path, &json_string).unwrap();

            // Return the fetched data, or handle as needed
            Ok::<Todo, reqwest::Error>(todo)
            // Ok::<(), reqwest::Error>(())
        });
        tasks.push((i, handle));
    }

    // Wait for all tasks to complete
    println!("Waiting for tasks to complete...");
    for (_i, handle) in tasks {
        // `handle.await` => Result<Result<Todo, reqwest::Error>, JoinError>
        // So we use `?` twice:
        // 1) once to handle JoinError,
        // 2) again to handle the potential reqwest::Error
        let _todo = handle.await??;
        // Optional: Print or process the result
        // println!("Task #{:4} => {:?}", i, todo);
    }
    println!("All tasks completed!");

    // Print the total time
    let duration = start_time.elapsed();
    println!("Total time: {:?}", duration);
    println!("{:?} req/s", count as f32 / duration.as_secs_f32());

    println!("Cleaning up...");
    // fs::remove_dir_all(path)?;

    Ok(())
}
