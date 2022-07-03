# Running on Pi Zero

Download and install speedtest CLI

    wget https://install.speedtest.net/app/cli/ookla-speedtest-1.1.1-linux-armel.tgz
    tar ztvf ookla-speedtest-1.1.1-linux-armel.tgz

    # do a test run (and accept licenses)
    ./speedtest

Setup Rust for local compilation

    # run installer (use defaults)
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # this is needed for compiling openssl support:
    sudo apt-get install libssl-dev

Setup Git and checkout code

    git clone https://github.com/mikemoraned/canhaveinternet.git

Compile and run the code

    source $HOME/.cargo/env
    cd canhaveinternet/snitch/snitch-agent

    # as well as building the code, this will do a one-off download of the crates.io index as well as all the crates needed
    cargo build

    # this runs the service, passing the path to the `speedtest` and `pizero1` as the agent name
    cargo run $HOME/speedtest pizero1
