use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let count = 500;

    let url = "http://jsonplaceholder.typicode.com/todos/1";

    // Build a Reqwest client with certificate validation disabled.
    // This is insecure in production!
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // <-- Disables certificate validation
        .redirect(reqwest::redirect::Policy::none())
        .danger_accept_invalid_hostnames(true) // (Optional) Disable hostname verification
        .build()?;

    // 2. Measure the total time
    let start_time = Instant::now();

    // Spawn concurrent tasks
    let mut tasks = Vec::with_capacity(1000);
    for i in 1..=count {
        let url_clone = url.to_string();
        let handle = tokio::spawn(async move {
            let response = client.get(&url_clone).await?;
            let todo: Todo = response.json().await?;
            // println!("{todo:?}");

            // Convert the struct to a pretty JSON string
            let json_string = serde_json::to_string_pretty(&todo).unwrap();
            // Save each response to /tmp/todo-#.json
            let path = format!("/tmp/todo-{}.json", i);
            fs::write(&path, &json_string).unwrap();

            // Return the fetched data, or handle as needed
            Ok::<Todo, reqwest::Error>(todo)
            // Ok::<(), reqwest::Error>(())
        });
        tasks.push((i, handle));
    }

    // Wait for all tasks to complete
    for (_, handle) in tasks {
        // `handle.await` => Result<Result<Todo, reqwest::Error>, JoinError>
        // So we use `?` twice:
        // 1) once to handle JoinError,
        // 2) again to handle the potential reqwest::Error
        let _ = handle.await??;
        // Optional: Print or process the result
        // println!("Task #{:4} => {:?}", i, todo);
    }

    // 4. Print the total time
    let duration = start_time.elapsed();
    println!(
        "rps: {:?}",
        (count as f32 / duration.as_millis() as f32) * 1000 as f32
    );

    Ok(())
}
