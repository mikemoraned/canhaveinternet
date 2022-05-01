# Running on Ubuntu Touch

... set up ssh (see https://docs.ubports.com/en/latest/userguide/advanceduse/ssh.html)

Set up base installation:

    # this may take some time as it downloads and installs a lot
    libertine-container-manager create -i snitch-agent

    # this gets rid of an annoying warning about "ERROR: ld.so: object 'libtls-padding.so' from LD_PRELOAD cannot be preloaded (cannot open shared object file): ignored." when you open a bash shell later:
    libertine-container-manager install-package -i snitch-agent -p tls-padding

Download and install speedtest CLI

    DISPLAY= libertine-launch -i snitch-agent /bin/bash
    wget https://install.speedtest.net/app/cli/ookla-speedtest-1.1.1-linux-aarch64.tgz
    tar zxvf ookla-speedtest-1.1.1-linux-aarch64.tgz

    # do a test run (and accept licenses)
    ./speedtest

Setup Rust for local compilation

    libertine-container-manager install-package -i snitch-agent -p curl
    DISPLAY= libertine-launch -i snitch-agent /bin/bash
    # run installer (use defaults)
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    # this is needed for the C compiler:
    libertine-container-manager install-package -i snitch-agent -p build-essential
    # this is needed for compiling openssl support:
    libertine-container-manager install-package -i snitch-agent -p pkg-config
    libertine-container-manager install-package -i snitch-agent -p libssl-dev

Setup Git and checkout code

    libertine-container-manager install-package -i snitch-agent -p git
    DISPLAY= libertine-launch -i snitch-agent /bin/bash
    git clone https://github.com/mikemoraned/canhaveinternet.git
    git checkout snitch-agent

Compile and run the code

    source $HOME/.cargo/env
    cd canhaveinternet/snitch/snitch-agent

    # as well as building the code, this will do a one-off download of the crates.io index as well as all the crates needed
    cargo build

    # this runs the service, using `./speedtest` as the path:
    cargo run /home/phablet/speedtest
