# What this is

1. I was getting suspicious about occasional latency from my home network to outside internet and wanted some evidence
2. I fancied learning some more Rust (`async-std` in particular) and also in setting up and using Prometheus for metrics

## Running Metrics Server

    docker run -p 9090:9090 -v `pwd`/prometheus.yml:/etc/prometheus/prometheus.yml prom/prometheus

## Running Clients

    cargo build --release

Then, copy `target/release/canhaveinternet` to the machine you want to run it on, and start it:

    ./canhaveinternet

You should see something like:

    Server is listening on: http://127.0.0.1:8000
    status = 200, start = 1573954064.760746s, elapsed = 174.566ms

Add the name of machine you installed it on to `static_configs`/`targets`. You should then see:

    dump called
