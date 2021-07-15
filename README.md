# Burst 💥

Burst is a simple tool that sends load to a host.

_WARNING: Burst is under active development. Subcommands and flags may change at any moment._

## How to use

```console
burst 0.1-dev
Sends bursts of requests to a specified host.

USAGE:
    burst [FLAGS] [OPTIONS] --host <host>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose mode.

OPTIONS:
    -d, --duration <duration>    Sends load for the given amount of time set in seconds.
                                 The actual running time will vary depending on the load, workers and the time it takes
                                 for the response to return.
    -h, --host <host>            Host header to send the requests to.
    -i, --interval <interval>    Interval time between bursts of requests in seconds. Requires --duration to be set.
    -l, --load <load>            Amount of requests to send. [default: 100]
    -p, --pass <pass>            Password for basic authentication.
    -t, --timeout <timeout>      Timeout in seconds for each request. [default: 20]
    -u, --user <user>            User for basic authentication.
    -w, --workers <workers>      Number of workers to run in parallel. [default: 10]
```

## Examples

Send 300 requests with 5 workers running in parallel:

```console
$ burst -l 300 -w 5
```

Send a single request continuously during 30 seconds:

```console
$ burst -l 1 -d 30
```

Send bursts of 20 requests every 2 seconds during 60 seconds:
```console
$ burst -l 20 -d 60 -i 2
```
