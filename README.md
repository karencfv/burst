# Burst

Burst is a simple tool to send load to a host.

## How to use

```console
burst 0.1-dev
Sends bursts of requests

USAGE:
    burst [OPTIONS] --host <host> --load <load> --timeout <timeout> --workers <workers>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>          Host header to send the requests to. [default: https://www.google.com/]
    -l, --load <load>          Amount of requests to send. [default: 100]
    -p, --pass <pass>          Password for basic authentication.
    -t, --timeout <timeout>    Timeout in seconds for each request. [default: 20]
    -u, --user <user>          User for basic authentication.
    -w, --workers <workers>    Number of workers to run in parallel. [default: 10]
```
