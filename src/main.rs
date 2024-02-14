use reqwest::header::{CONNECTION, USER_AGENT};
use reqwest::{Client, ClientBuilder};
use std::error::Error;
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let stdin = io::stdin();

    let client = ClientBuilder::new().danger_accept_invalid_certs(true).build()?;
    let mut handles = vec![];

    for line in stdin.lock().lines() {
        let line = line?;

        // println!("line:{}",line);
        let address = format!("https://{}", line);
        let client_clone = client.clone(); // Clone the client here

        let handle = tokio::spawn(async move {
            if is_https_listen(&client_clone, &address).await {
                println!("{}", address);
            }
            let htpad = format!("http://{}",line);
            if is_http_listen(&client_clone, &htpad).await {
                println!("{}", htpad);
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
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59")
        .header(CONNECTION, "close")
        .send();

    match timeout(Duration::from_secs(10), res).await {
        Ok(Ok(res)) => res.status().is_success(),
        Ok(Err(_)) => false, // The request itself failed (e.g., DNS resolution error, connectivity error)
        Err(_) => false,
    }
}

async fn is_http_listen(client: &Client, address: &str) ->bool {
    let res = client
        .get(address)
        .header(CONNECTION, "close")
        .send();

    match timeout(Duration::from_secs(20), res).await {
        Ok(Ok(res)) => res.status().is_success(),
        Ok(Err(_)) => false, // The request itself failed (e.g., DNS resolution error, connectivity error)
        Err(_) => false,
    }
}


