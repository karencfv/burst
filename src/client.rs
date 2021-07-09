use futures::{stream, StreamExt};
use rand::Rng;
use reqwest::{Method, Result};
use usdt::dtrace_provider;

use std::time::{Duration, Instant};

dtrace_provider!("src/burst.d");

#[derive(Clone, Debug)]
pub struct Client {
    pub req_client: reqwest::Client,
    pub requests: Vec<usize>,
    pub duration: u64,
    pub host: String,
    pub workers: usize,
    pub method: Method,
    pub basic_auth: (String, Option<String>),
}

impl Client {
    pub fn new(
        requests: Vec<usize>,
        duration: u64,
        host: String,
        workers: usize,
        timeout: u64,
        method: Method,
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
            duration,
            host,
            workers,
            method,
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

    // I'm not sure if these methods that consume other methods should be
    // standalone functions that take Client as a parameter instead.
    // Which would be idiomatic Rust?
    pub async fn send_load(&self) {
        if self.duration > 0 {
            // maybe remove this line?
            println!("Sending requests for {} seconds...", self.duration);
            self.process_requests_duration().await;
        } else {
            // maybe remove this line?
            println!("Sending {} requests...", self.requests.len());
            self.process_requests().await;
        }
    }

    pub async fn process_requests(&self) {
        let id = rand::thread_rng().gen();
        burst_requests__start!(|| id);

        let requests = stream::iter(&self.requests)
            .map(|_| {
                let client = self.clone();
                tokio::spawn(async move {
                    match &client.method {
                        &Method::GET => {
                            if let Err(e) = client.get().await {
                                eprintln!("Request error: {}", e);
                            }
                        }
                        // TODO: Implement other methods
                        _ => panic!("{} is not a supported HTTP method", client.method),
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

    pub async fn process_requests_duration(&self) {
        let mut client = self.clone();
        client.requests = vec![1];
        let secs = client.duration;

        let now = Instant::now();
        while now.elapsed().as_secs() < secs {
            client.process_requests().await;
        }
    }
}
