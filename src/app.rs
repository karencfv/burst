use clap::{App, Arg};
use reqwest::Method;

use crate::client::Client;

pub fn burst_app() -> Client {
    let app = App::new("burst")
        .version("0.1-dev")
        .about("Sends bursts of requests to a specified host.");

    let load_arg = Arg::with_name("load")
        .long("load")
        .short("l")
        .takes_value(true)
        .default_value("100")
        .help("Amount of requests to send.")
        .required(false);

    let workers_arg = Arg::with_name("workers")
        .long("workers")
        .short("w")
        .takes_value(true)
        .default_value("10")
        .help("Number of workers to run in parallel.")
        .required(false);

    let duration_arg = Arg::with_name("duration")
        .long("duration")
        .short("d")
        .takes_value(true)
        .help("Sends load for the given amount of time set in seconds.
The actual running time will vary depending on the load, workers and the time it takes for the response to return.")
        .required(false);

    let interval_arg = Arg::with_name("interval")
        .long("interval")
        .short("i")
        .takes_value(true)
        .requires("duration")
        .help("Interval time between bursts of requests in seconds. Requires --duration to be set.")
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
        .help("Host header to send the requests to.")
        .required(true);

    let user_arg = Arg::with_name("user")
        .long("user")
        .short("u")
        .takes_value(true)
        .help("User for basic authentication.")
        .required(false);

    let pass_arg = Arg::with_name("pass")
        .long("pass")
        .short("p")
        .takes_value(true)
        .help("Password for basic authentication.")
        .required(false);

    let verbose_arg = Arg::with_name("verbose")
        .long("verbose")
        .short("v")
        .help("Enable verbose mode.")
        .required(false);

    let app = app
        .arg(load_arg)
        .arg(duration_arg)
        .arg(interval_arg)
        .arg(timeout_arg)
        .arg(host_arg)
        .arg(workers_arg)
        .arg(pass_arg)
        .arg(verbose_arg)
        .arg(user_arg);

    let matches = app.get_matches();

    let load = matches
        .value_of("load")
        .expect("A value for load is required.");

    let workers = matches
        .value_of("workers")
        .expect("A value for workers is required.");

    let timeout = matches
        .value_of("timeout")
        .expect("A value for timeout is required.");

    let host = matches
        .value_of("host")
        .expect("A value for host is required.");

    let workers: usize = workers.parse().unwrap();
    let timeout: u64 = timeout.parse().unwrap();

    let reqs: usize = load.parse().unwrap();
    let requests: Vec<usize> = (0..reqs).collect();

    // Sets duration and interval to 0 unless the duration subcommand has been used.
    // In which case load will be sent for a specified duration as opposed
    // to a single burst of n amount of requests.
    let mut duration: u64 = 0;
    if matches.is_present("duration") {
        let time = matches
            .value_of("duration")
            .expect("A value for time is required.");
        let time: u64 = time.parse().unwrap();
        duration = time;
    }

    let mut interval: u64 = 0;
    if matches.is_present("interval") {
        let interval_time = matches
            .value_of("interval")
            .expect("A value for interval is required");
        let interval_time: u64 = interval_time.parse().unwrap();
        interval = interval_time;
    }

    let mut user = String::from("");
    if matches.is_present("user") {
        let user_str = matches
            .value_of("user")
            .expect("A value for user is required");
        let user_str: String = user_str.parse().unwrap();
        user = user_str;
    }

    let mut pass = Some(String::from(""));
    if matches.is_present("pass") {
        let pass_str = matches
            .value_of("pass")
            .expect("A value for pass is required");
        let pass_str: String = pass_str.parse().unwrap();
        pass = Some(pass_str);
    }

    let mut verbose = false;
    if matches.is_present("verbose") {
        verbose = true;
    }

    // For now hardcoding this to GET, but will introduce other HTTP methods eventually
    let method = Method::GET;

    // TODO: For now user and pass are being sent as empty strings if empty.
    // I need to figure out how to use Option<T> to send these as None.
    Client::new(
        requests,
        duration,
        interval,
        String::from(host),
        workers,
        timeout,
        method,
        user,
        pass,
        verbose,
    )
}
