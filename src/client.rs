use futures::{stream, StreamExt};
use rand::Rng;
use reqwest::{Method, Result};
use usdt::dtrace_provider;

use std::arch::asm;
use std::time::{Duration, Instant};
use std::{process, thread, time};

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
    pub body: String,
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
        exact: bool,
        host: String,
        workers: usize,
        timeout: u64,
        method: Method,
        body: String,
        user: String,
        pass: Option<String>,
        verbose: bool,
    ) -> Self {
        let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let req_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .user_agent(user_agent)
            .build()
            .expect("Unable to build client");

        let kind = kind_match(&duration, &exact);

        Self {
            req_client,
            requests,
            duration,
            interval,
            host,
            workers,
            method,
            body,
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
                println!("Sending {} requests...", self.requests.len());
                self.process_requests(id).await;
            }
            Kind::Timed => {
                println!("Sending requests for {} seconds...", self.duration);

                self.process_requests_timed(id).await;
            }
            Kind::TimedExact => {
                println!(
                    "Sending requests and will exit in {} seconds...",
                    self.duration
                );

                self.process_requests_timed_exact(id).await;
            }
        }
    }

    async fn get(&self) -> Result<()> {
        let id: u64 = rand::thread_rng().gen();
        burst::get__start!(|| id);

        let res = self
            .req_client
            .get(&self.host)
            .basic_auth(&self.user, self.pass.as_ref())
            .send()
            .await?;
        burst::get__done!(|| id);

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

    async fn post(&self) -> Result<()> {
        let id: u64 = rand::thread_rng().gen();
        burst::post__start!(|| id);

        let res = self
            .req_client
            .post(&self.host)
            .basic_auth(&self.user, self.pass.as_ref())
            .body(self.body.to_string())
            .send()
            .await?;

        burst::post__done!(|| id);

        if self.verbose {
            println!("Request ID: {} status: {}", id, res.status());
        }

        Ok(())
    }

    async fn put(&self) -> Result<()> {
        let id: u64 = rand::thread_rng().gen();
        burst::put__start!(|| id);

        let res = self
            .req_client
            .put(&self.host)
            .basic_auth(&self.user, self.pass.as_ref())
            .body(self.body.to_string())
            .send()
            .await?;

        burst::put__done!(|| id);

        if self.verbose {
            println!("Request ID: {} status: {}", id, res.status());
        }

        Ok(())
    }

    async fn patch(&self) -> Result<()> {
        let id: u64 = rand::thread_rng().gen();
        burst::patch__start!(|| id);

        let res = self
            .req_client
            .patch(&self.host)
            .basic_auth(&self.user, self.pass.as_ref())
            .body(self.body.to_string())
            .send()
            .await?;

        burst::patch__done!(|| id);

        if self.verbose {
            println!("Request ID: {} status: {}", id, res.status());
        }

        Ok(())
    }

    async fn process_requests(&self, id: u64) {
        burst::requests__start!(|| id);

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
                        Method::POST => {
                            if let Err(e) = client.post().await {
                                eprintln!("Request error: {}", e);
                            }
                        }
                        Method::PUT => {
                            if let Err(e) = client.put().await {
                                eprintln!("Request error: {}", e);
                            }
                        }
                        Method::PATCH => {
                            if let Err(e) = client.patch().await {
                                eprintln!("Request error: {}", e);
                            }
                        }
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

        burst::requests__done!(|| id);
    }

    async fn process_requests_timed(&self, id: u64) {
        let now = Instant::now();

        if self.interval > 0 {
            let mut interval = tokio::time::interval(time::Duration::from_secs(self.interval));

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
    }

    async fn process_requests_timed_exact(&self, id: u64) {
        let duration = self.duration;
        thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(duration));
            process::exit(0);
        });

        if self.interval > 0 {
            let mut interval = tokio::time::interval(time::Duration::from_secs(self.interval));

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

fn kind_match(duration: &u64, exact: &bool) -> Kind {
    match exact {
        false if { duration > &0 } => Kind::Timed,
        true if { duration > &0 } => Kind::TimedExact,
        _ => Kind::Single,
    }
}
