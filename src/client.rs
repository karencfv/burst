use futures::{stream, StreamExt};
use rand::Rng;
use reqwest::Result;
use usdt::dtrace_provider;

use std::time::Duration;

dtrace_provider!("src/burst.d");

#[derive(Clone, Debug)]
pub struct Client {
    pub req_client: reqwest::Client,
    pub requests: Vec<usize>,
    pub host: String,
    pub workers: usize,
    pub basic_auth: (String, Option<String>),
}

impl Client {
    pub fn new(
        requests: Vec<usize>,
        host: String,
        workers: usize,
        timeout: u64,
        user: String,
        pass: String,
    ) -> Self {
        let basic_auth = (user, Some(pass));
        let req_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .expect("Unable to build client");
        Self {
            req_client,
            requests,
            host,
            workers,
            basic_auth,
        }
    }

    pub async fn get(&self) -> Result<()> {
        let id = rand::thread_rng().gen();
        let (u, p) = &self.basic_auth;
        burst_get__start!(|| id);

        let res = self
            .req_client
            .get(&self.host)
            .basic_auth(u, p.as_ref())
            .send()
            .await?;
        burst_get__done!(|| id);

        // TODO: Only print with --verbose. Also, maybe instead of printing all statuses,
        // create a summary of how many requests returned each status.
        println!("Request ID: {} status: {}", id, res.status());
        // The following prints the response body. Might be useful to have as an
        // option to print it all out in a file.
        // let body = res.text().await?;
        // println!("Body:\n\n{}", body);
        Ok(())
    }

    pub async fn process_requests(&self) {
        let id = rand::thread_rng().gen();
        burst_requests__start!(|| id);

        let requests = stream::iter(&self.requests)
            .map(|_| {
                let client = self.clone();
                tokio::spawn(async move {
                    if let Err(e) = client.get().await {
                        eprintln!("Request error: {}", e);
                    }
                })
            })
            .buffer_unordered(self.workers);

        requests
            .for_each(|b| async {
                if let Err(e) = b {
                    eprintln!("Internal tokio::JoinError: {}", e)
                };
            })
            .await;

        burst_requests__done!(|| id);
    }
}
