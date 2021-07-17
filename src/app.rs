use clap::{App, Arg};
use reqwest::Method;

use crate::client::Client;

//#[macro_use]
macro_rules! validate_flag_error {
    ($flag:tt) => {
        &format!("A value for {} is required", $flag);
    };
}

pub fn burst_app() -> Client {
    let app = App::new("burst")
        .version("0.1-dev")
        .about("Sends bursts of requests to a specified host.");

    let load_flag = "load";
    let load_arg = Arg::with_name(load_flag)
        .long(load_flag)
        .short("l")
        .takes_value(true)
        .default_value("100")
        .help("Amount of requests to send.")
        .required(false);

    let workers_flag = "workers";
    let workers_arg = Arg::with_name(workers_flag)
        .long(workers_flag)
        .short("w")
        .takes_value(true)
        .default_value("10")
        .help("Number of workers to run in parallel.")
        .required(false);

    let duration_flag = "duration";
    let duration_arg = Arg::with_name(duration_flag)
        .long(duration_flag)
        .short("d")
        .takes_value(true)
        .help("Sends load for the given amount of time set in seconds.
The actual running time will vary depending on the load, workers and the time it takes for the response to return.")
        .required(false);

    let interval_flag = "interval";
    let interval_arg = Arg::with_name(interval_flag)
        .long(interval_flag)
        .short("i")
        .takes_value(true)
        .requires(duration_flag)
        .help("Interval time between bursts of requests in seconds. Requires --duration to be set.")
        .required(false);

    let timeout_flag = "timeout";
    let timeout_arg = Arg::with_name(timeout_flag)
        .long(timeout_flag)
        .short("t")
        .takes_value(true)
        .default_value("20")
        .help("Timeout in seconds for each request.")
        .required(false);

    let host_flag = "host";
    let host_arg = Arg::with_name(host_flag)
        .long(host_flag)
        .short("h")
        .takes_value(true)
        .help("Host header to send the requests to.")
        .required(true);

    let user_flag = "user";
    let user_arg = Arg::with_name(user_flag)
        .long(user_flag)
        .short("u")
        .takes_value(true)
        .help("User for basic authentication.")
        .required(false);

    let pass_flag = "pass";
    let pass_arg = Arg::with_name(pass_flag)
        .long(pass_flag)
        .short("p")
        .takes_value(true)
        .help("Password for basic authentication.")
        .required(false);

    let verbose_flag = "verbose";
    let verbose_arg = Arg::with_name(verbose_flag)
        .long(verbose_flag)
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
        .value_of(load_flag)
        .expect(validate_flag_error!(load_flag));

    let workers = matches
        .value_of(workers_flag)
        .expect(validate_flag_error!(workers_flag));

    let timeout = matches
        .value_of(timeout_flag)
        .expect(validate_flag_error!(timeout_flag));

    let host = matches
        .value_of(host_flag)
        .expect(validate_flag_error!(host_flag));

    let workers: usize = workers.parse().unwrap();
    let timeout: u64 = timeout.parse().unwrap();

    let reqs: usize = load.parse().unwrap();
    let requests: Vec<usize> = (0..reqs).collect();

    // Sets duration and interval to 0 unless the duration flag has been used.
    // In which case load will be sent for a specified duration as opposed
    // to a single burst of n amount of requests.
    let mut duration: u64 = 0;
    if matches.is_present(duration_flag) {
        let time = matches
            .value_of(duration_flag)
            .expect(validate_flag_error!(duration_flag));
        let time: u64 = time.parse().unwrap();
        duration = time;
    }

    let mut interval: u64 = 0;
    if matches.is_present(interval_flag) {
        let interval_time = matches
            .value_of(interval_flag)
            .expect(validate_flag_error!(interval_flag));
        let interval_time: u64 = interval_time.parse().unwrap();
        interval = interval_time;
    }

    let mut user = String::from("");
    if matches.is_present(user_flag) {
        let user_str = matches
            .value_of(user_flag)
            .expect(validate_flag_error!(user_flag));
        let user_str: String = user_str.parse().unwrap();
        user = user_str;
    }

    let mut pass = None;
    if matches.is_present(pass_flag) {
        let pass_str = matches
            .value_of(pass_flag)
            .expect(validate_flag_error!(pass_flag));
        let pass_str: String = pass_str.parse().unwrap();
        pass = Some(pass_str);
    }

    let mut verbose = false;
    if matches.is_present(verbose_flag) {
        verbose = true;
    }

    // For now hardcoding this to GET, but will introduce other HTTP methods eventually
    let method = Method::GET;

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
