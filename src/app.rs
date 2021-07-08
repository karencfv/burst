use clap::{App, Arg};

use crate::client::Client;

pub fn burst_app() -> Client {
    let app = App::new("burst")
        .version("0.1-dev")
        .about("Sends bursts of requests");

    let load_arg = Arg::with_name("load")
        .long("load")
        .short("l")
        .takes_value(true)
        .default_value("100")
        .help("Amount of requests to send.")
        .required(false);

    let duration_arg = Arg::with_name("duration")
        .long("duration")
        .short("d")
        .takes_value(true)
        .default_value("0")
        .help("Sends load for the given amount of time set in seconds. Cannot be used in conjunction with --load")
        .required(false);

    let workers_arg = Arg::with_name("workers")
        .long("workers")
        .short("w")
        .takes_value(true)
        .default_value("10")
        .help("Number of workers to run in parallel.")
        .required(false);

    let timeout_arg = Arg::with_name("timeout")
        .long("timeout")
        .short("t")
        .takes_value(true)
        .default_value("20")
        .help("Timeout in seconds for each request.")
        .required(false);

    let host_arg = Arg::with_name("host")
        .long("host")
        .short("h")
        .takes_value(true)
        .default_value("https://www.google.com/") // Only for development, remove later
        .help("Host header to send the requests to.")
        .required(true);

    let user_arg = Arg::with_name("user")
        .long("user")
        .short("u")
        .takes_value(true)
        .default_value("")
        .help("User for basic authentication.")
        .required(false);

    let pass_arg = Arg::with_name("pass")
        .long("pass")
        .short("p")
        .takes_value(true)
        .default_value("")
        .help("Password for basic authentication.")
        .required(false);

    let app = app
        .arg(load_arg)
        .arg(duration_arg)
        .arg(timeout_arg)
        .arg(host_arg)
        .arg(workers_arg)
        .arg(pass_arg)
        .arg(user_arg);

    let matches = app.get_matches();
    value_t!(matches, "load", usize).expect("Value for load must be a positive number");
    value_t!(matches, "duration", usize).expect("Value for duration must be a positive number");
    value_t!(matches, "workers", usize).expect("Value for workers must be a positive number");
    value_t!(matches, "timeout", usize).expect("Value for timeout must be a positive number");

    let load = matches
        .value_of("load")
        .expect("A value for load is required");

    let duration = matches
        .value_of("duration")
        .expect("A value for duration is required");

    let workers = matches
        .value_of("workers")
        .expect("A value for workers is required");

    let timeout = matches
        .value_of("timeout")
        .expect("A value for timeout is required");

    let host = matches
        .value_of("host")
        .expect("A value for host is required");

    let user = matches
        .value_of("user")
        .expect("A value for user is required");

    let pass = matches
        .value_of("pass")
        .expect("A value for pass is required");

    let duration: u64 = duration.parse().unwrap();
    let workers: usize = workers.parse().unwrap();
    let timeout: u64 = timeout.parse().unwrap();
    let user: String = user.parse().unwrap();
    let pass: String = pass.parse().unwrap();

    let reqs: usize = load.parse().unwrap();
    let requests: Vec<usize> = (0..reqs).collect();

    // TODO: For now user and pass are being sent as empty strings.
    // I need to figure out how to use Option<T> to send these as None if empty.
    Client::new(
        requests,
        duration,
        String::from(host),
        workers,
        timeout,
        user,
        pass,
    )
}
