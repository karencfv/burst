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
    -e, --exact      Starts a timer when using --duration. This means that the running time will be exact to the set
                     duration time, but some requests may have not completed.
        --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose mode.

OPTIONS:
    -b, --body <body>            HTTP request body.
    -d, --duration <duration>    Sends load for the given amount of time set in seconds.
                                 The actual running time will vary depending on the load, workers and the time it takes
                                 for the response to return.
    -h, --host <host>            Host header to send the requests to.
    -i, --interval <interval>    Interval time between bursts of requests in seconds. Requires --duration to be set.
    -l, --load <load>            Amount of requests to send. [default: 100]
    -m, --method <method>        HTTP method for request. One of 'get', 'post', 'put', or 'patch'. [default: get]
    -p, --pass <pass>            Password for basic authentication.
    -t, --timeout <timeout>      Timeout in seconds for each request. [default: 20]
    -u, --user <user>            User for basic authentication.
    -w, --workers <workers>      Number of workers to run in parallel. [default: 10]
```

## Examples

Send 300 requests with 5 workers running in parallel:

```console
$ burst -h http://127.0.0.1 -l 300 -w 5
```

Send a single request continuously during 30 seconds with a timeout of 5 seconds for each request:

```console
$ burst -h http://127.0.0.1 -l 1 -d 30
```

Send bursts of 20 requests every 2 seconds during 60 seconds:
```console
$ burst -h http://127.0.0.1 -l 20 -d 60 -i 2 -t 5
```

Send bursts of 15 requests continuously with a set timer for 10 seconds (Some requests may not have time to send a response back):
```console
$ burst -h http://127.0.0.1 -l 15 -d 10 -e
```

Send a single PUT request:
```console
$ burst -h http://127.0.0.1 -l 1 -m put -b '{"some_key":"some_value"}'
```

## Using DTrace

Burst has [four probes](./src/burst.d) available which can be leveraged in conjunction with DTrace to retrieve useful information. 

Try the sample script found in [scripts/](./scripts/request_lifetime.sh) while burst is running, as shown below:

```console
$ sudo ./scripts/request_lifetime.sh 

Summary of all GET request round trips taken in ten seconds represented in nanoseconds:
  total requests                                                 1344
  average request lifetime                                   13714645
  max request lifetime                                       43921416
  min request lifetime                                         888625
  request lifetimes visualisation                   
           value  ------------- Distribution ------------- count    
          262144 |                                         0        
          524288 |@@@                                      100      
         1048576 |@@@                                      104      
         2097152 |@@                                       64       
         4194304 |@@@@@                                    170      
         8388608 |@@@@@@@@@@@@@                            424      
        16777216 |@@@@@@@@@@@@@                            441      
        33554432 |@                                        41       
        67108864 |                                         0  
```

To find more information on using DTrace, visit the official [Dynamic Tracing Guide](https://illumos.org/books/dtrace/preface.html#preface).
