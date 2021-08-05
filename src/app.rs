use clap::{App, Arg};
use reqwest::Method;

use crate::client::Client;

//#[macro_use]
macro_rules! validate_flag_error {
    ($flag:tt) => {
        &format!("A value for {} is required", $flag);
    };
}

const LOAD_FLAG: &str = "load";
const WORKERS_FLAG: &str = "workers";
const DURATION_FLAG: &str = "duration";
const INTERVAL_FLAG: &str = "interval";
const TIMEOUT_FLAG: &str = "timeout";
const METHOD_FLAG: &str = "method";
const BODY_FLAG: &str = "body";
const HOST_FLAG: &str = "host";
const USER_FLAG: &str = "user";
const PASS_FLAG: &str = "pass";
const EXACT_FLAG: &str = "exact";
const VERBOSE_FLAG: &str = "verbose";

fn cmd<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("burst")
        .version("0.1-dev")
        .about("Sends bursts of requests to a specified host.");

    let load_arg = Arg::with_name(LOAD_FLAG)
        .long(LOAD_FLAG)
        .short("l")
        .takes_value(true)
        .default_value("100")
        .help("Amount of requests to send.")
        .required(false);

    let workers_arg = Arg::with_name(WORKERS_FLAG)
        .long(WORKERS_FLAG)
        .short("w")
        .takes_value(true)
        .default_value("10")
        .help("Number of workers to run in parallel.")
        .required(false);

    let duration_arg = Arg::with_name(DURATION_FLAG)
        .long(DURATION_FLAG)
        .short("d")
        .takes_value(true)
        .help("Sends load for the given amount of time set in seconds.
The actual running time will vary depending on the load, workers and the time it takes for the response to return.")
        .required(false);

    let interval_arg = Arg::with_name(INTERVAL_FLAG)
        .long(INTERVAL_FLAG)
        .short("i")
        .takes_value(true)
        .requires(DURATION_FLAG)
        .help("Interval time between bursts of requests in seconds. Requires --duration to be set.")
        .required(false);

    let timeout_arg = Arg::with_name(TIMEOUT_FLAG)
        .long(TIMEOUT_FLAG)
        .short("t")
        .takes_value(true)
        .default_value("20")
        .help("Timeout in seconds for each request.")
        .required(false);

    let host_arg = Arg::with_name(HOST_FLAG)
        .long(HOST_FLAG)
        .short("h")
        .takes_value(true)
        .help("Host header to send the requests to.")
        .required(true);

    let method_arg = Arg::with_name(METHOD_FLAG)
        .long(METHOD_FLAG)
        .short("m")
        .takes_value(true)
        .help("HTTP method for request. One of 'get', 'post', 'put', or 'patch'.")
        .default_value("get")
        .required(false);

    let body_arg = Arg::with_name(BODY_FLAG)
        .long(BODY_FLAG)
        .short("b")
        .takes_value(true)
        .help("HTTP request body.")
        .required(false);

    let user_arg = Arg::with_name(USER_FLAG)
        .long(USER_FLAG)
        .short("u")
        .takes_value(true)
        .help("User for basic authentication.")
        .required(false);

    let pass_arg = Arg::with_name(PASS_FLAG)
        .long(PASS_FLAG)
        .short("p")
        .takes_value(true)
        .help("Password for basic authentication.")
        .required(false);

    let exact_arg = Arg::with_name(EXACT_FLAG)
        .long(EXACT_FLAG)
        .short("e")
        .requires(DURATION_FLAG)
        .help("Starts a timer when using --duration. This means that the running time will be exact to the set duration time, but some requests may have not completed.")
        .required(false);

    let verbose_arg = Arg::with_name(VERBOSE_FLAG)
        .long(VERBOSE_FLAG)
        .short("v")
        .help("Enable verbose mode.")
        .required(false);

    app.arg(load_arg)
        .arg(duration_arg)
        .arg(interval_arg)
        .arg(exact_arg)
        .arg(timeout_arg)
        .arg(host_arg)
        .arg(workers_arg)
        .arg(pass_arg)
        .arg(verbose_arg)
        .arg(user_arg)
        .arg(method_arg)
        .arg(body_arg)
}

pub fn burst_app() -> Client {
    let app = cmd();

    let matches = app.get_matches();

    let load = matches
        .value_of(LOAD_FLAG)
        .expect(validate_flag_error!(LOAD_FLAG));

    let workers = matches
        .value_of(WORKERS_FLAG)
        .expect(validate_flag_error!(WORKERS_FLAG));

    let timeout = matches
        .value_of(TIMEOUT_FLAG)
        .expect(validate_flag_error!(TIMEOUT_FLAG));

    let host = matches
        .value_of(HOST_FLAG)
        .expect(validate_flag_error!(HOST_FLAG));
    let host: String = host.parse().unwrap();

    let method = matches
        .value_of(METHOD_FLAG)
        .expect(validate_flag_error!(METHOD_FLAG));

    let workers: usize = workers.parse().unwrap();
    let timeout: u64 = timeout.parse().unwrap();

    let reqs: usize = load.parse().unwrap();
    let requests: Vec<usize> = (0..reqs).collect();

    // Sets duration and interval to 0 unless the duration flag has been used.
    // In which case load will be sent for a specified duration as opposed
    // to a single burst of n amount of requests.
    let duration: u64 = if matches.is_present(DURATION_FLAG) {
        let time = matches
            .value_of(DURATION_FLAG)
            .expect(validate_flag_error!(DURATION_FLAG));
        let time: u64 = time.parse().unwrap();
        time
    } else {
        0
    };

    let interval: u64 = if matches.is_present(INTERVAL_FLAG) {
        let interval_time = matches
            .value_of(INTERVAL_FLAG)
            .expect(validate_flag_error!(INTERVAL_FLAG));
        let interval_time: u64 = interval_time.parse().unwrap();
        interval_time
    } else {
        0
    };

    let exact = if matches.is_present(EXACT_FLAG) {
        true
    } else {
        false
    };

    let user = if matches.is_present(USER_FLAG) {
        let user_str = matches
            .value_of(USER_FLAG)
            .expect(validate_flag_error!(USER_FLAG));
        let user_str: String = user_str.parse().unwrap();
        user_str
    } else {
        String::from("")
    };

    let pass = if matches.is_present(PASS_FLAG) {
        let pass_str = matches
            .value_of(PASS_FLAG)
            .expect(validate_flag_error!(PASS_FLAG));
        let pass_str: String = pass_str.parse().unwrap();
        Some(pass_str)
    } else {
        None
    };

    let body = if matches.is_present(BODY_FLAG) {
        let body_str = matches
            .value_of(BODY_FLAG)
            .expect(validate_flag_error!(BODY_FLAG));
        let body_str: String = body_str.parse().unwrap();
        body_str
    } else {
        String::from("")
    };

    let verbose = if matches.is_present(VERBOSE_FLAG) {
        true
    } else {
        false
    };

    let http_method: Method;

    match method {
        "get" => {
            http_method = Method::GET;
        }
        "post" => {
            http_method = Method::POST;
        }
        "put" => {
            http_method = Method::PUT;
        }
        "patch" => {
            http_method = Method::PATCH;
        }
        _ => panic!(
            "{} is not a supported HTTP method. Use one of: 'get', 'post', 'put', or 'patch'.",
            method
        ),
    };

    // For now hardcoding this to GET, but will introduce other HTTP methods eventually
    // let method = Method::GET;

    Client::new(
        requests,
        duration,
        interval,
        exact,
        host,
        workers,
        timeout,
        http_method,
        body,
        user,
        pass,
        verbose,
    )
}
