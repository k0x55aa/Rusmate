use reqwest::header::CONNECTION;
use reqwest::Client;
use std::error::Error;
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let stdin = io::stdin();

    let client = Client::new();
    let mut handles = vec![];

    for line in stdin.lock().lines() {
        let line = line?;

        // println!("line:{}",line);
        let address = format!("http://{}", line);
        let client_clone = client.clone(); // Clone the client here

        let handle = tokio::spawn(async move {
            if is_https_listen(&client_clone, &address).await {
                println!("{:?}", address);
            } else {
               let stripurl = address.strip_prefix("http://").unwrap();
               let htpad = format!("https://{}",stripurl);
                if is_http_listen(&client_clone, &htpad).await {
                    println!("{:?}", htpad);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn is_https_listen(client: &Client, address: &str) ->bool {
    let res = client
        .get(address)
        .header(CONNECTION, "close")
        .send();

    match timeout(Duration::from_secs(20), res).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn is_http_listen(client: &Client, address: &str) ->bool {
    let res = client
        .get(address)
        .header(CONNECTION, "close")
        .send();

    match timeout(Duration::from_secs(20), res).await {
        Ok(_) => true,
        Err(_) => false,
    }
}