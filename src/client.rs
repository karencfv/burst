use futures::{stream, StreamExt};
use rand::Rng;
use reqwest::{Method, Result};
use tokio::time;
use usdt::dtrace_provider;

use std::process;
use std::time::{Duration, Instant};

dtrace_provider!("src/burst.d");

#[derive(Clone, Debug)]
pub enum Kind {
    Single,
    Timed,
    TimedExact,
}

#[derive(Clone, Debug)]
pub struct Client {
    pub req_client: reqwest::Client,
    pub requests: Vec<usize>,
    pub duration: u64,
    pub interval: u64,
    pub host: String,
    pub workers: usize,
    pub method: Method,
    pub user: String,
    pub pass: Option<String>,
    pub verbose: bool,
    pub kind: Kind,
}

impl Client {
    pub fn new(
        requests: Vec<usize>,
        duration: u64,
        interval: u64,
        host: String,
        workers: usize,
        timeout: u64,
        method: Method,
        user: String,
        pass: Option<String>,
        verbose: bool,
    ) -> Self {
        let req_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .expect("Unable to build client");

        let mut kind = Kind::Single;
        if duration > 0 {
            kind = Kind::Timed;
        }

        Self {
            req_client,
            requests,
            duration,
            interval,
            host,
            workers,
            method,
            user,
            pass,
            verbose,
            kind,
        }
    }

    // I'm not sure if these methods that execute other methods should be
    // standalone functions that take Client as a parameter instead.
    // Which would be idiomatic Rust?
    pub async fn send_load(&self) {
        let id: u64 = rand::thread_rng().gen();

        match self.kind {
            Kind::Single => {
                // maybe remove this line?
                println!("Sending {} requests...", self.requests.len());
                self.process_requests(id).await;
            }
            Kind::Timed => {
                // maybe remove this line?
                println!("Sending requests for {} seconds...", self.duration);
                self.process_requests_timed(id).await;
            }
            Kind::TimedExact => {
                println!("Not implemented yet");
            }
        }
    }

    async fn get(&self) -> Result<()> {
        let id = rand::thread_rng().gen();
        burst_get__start!(|| id);

        let res = self
            .req_client
            .get(&self.host)
            .basic_auth(&self.user, self.pass.as_ref())
            .send()
            .await?;
        burst_get__done!(|| id);

        // TODO: Maybe create a summary of how many requests returned each status as well?
        if self.verbose {
            println!("Request ID: {} status: {}", id, res.status());
        }

        // The following prints the response body. Might be useful to have as an
        // option to print it all out in a file?.
        // let body = res.text().await?;
        // println!("Body:\n\n{}", body);
        Ok(())
    }

    async fn process_requests(&self, id: u64) {
        burst_requests__start!(|| id);

        let requests = stream::iter(&self.requests)
            .map(|_| {
                let client = self.clone();
                tokio::spawn(async move {
                    match client.method {
                        Method::GET => {
                            if let Err(e) = client.get().await {
                                eprintln!("Request error: {}", e);
                            }
                        }
                        // TODO: Implement other methods
                        _ => eprintln!("{} is not a supported HTTP method", client.method),
                    }
                })
            })
            .buffer_unordered(self.workers);

        requests
            .for_each(|r| async {
                if let Err(e) = r {
                    eprintln!("Internal tokio::JoinError: {}", e);
                };
            })
            .await;

        burst_requests__done!(|| id);
    }

    async fn process_requests_timed(&self, id: u64) {
        let now = Instant::now();

        burst_timedrequests__start!(|| id);
        if self.interval > 0 {
            let mut interval = time::interval(time::Duration::from_secs(self.interval));

            while now.elapsed().as_secs() < self.duration {
                if self.verbose {
                    println!("Pausing for {} seconds", self.interval);
                }
                interval.tick().await;
                self.process_requests(id).await;
            }
        } else {
            while now.elapsed().as_secs() < self.duration {
                self.process_requests(id).await;
            }
        }
        burst_timedrequests__done!(|| id);
    }

    async fn process_requests_timed_exact(&'static self, id: u64) {
        if let Err(e) = tokio::task::spawn(async move {
            tokio::time::sleep(time::Duration::from_secs(self.duration)).await;
            process::exit(0);
        })
        .await
        {
            eprintln!("Internal task sleep error: {}", e);
            process::exit(1);
        }

        if self.interval > 0 {
            let mut interval = time::interval(time::Duration::from_secs(self.interval));

            loop {
                if self.verbose {
                    println!("Pausing for {} seconds", self.interval);
                }
                interval.tick().await;
                self.process_requests(id).await;
            }
        } else {
            loop {
                self.process_requests(id).await;
            }
        }
    }
}
