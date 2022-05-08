# InfluxDB (1.x) Setup

## Installation

Use https://docs.influxdata.com/influxdb/v2.2/install/ base and https://portal.influxdata.com/downloads/ as that sets it up via apt
as opposed to a direct .deb install.

    # update all existing software (optional)
    sudo apt update
    sudo apt upgrade

    # get key for signing
    wget -qO- https://repos.influxdata.com/influxdb.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/influxdb.gpg > /dev/null
    # curl https://repos.influxdata.com/influxdb.key | gpg --dearmor | sudo tee /usr/share/keyrings/influxdb-archive-keyring.gpg >/dev/null

    # add the repo
    #export DISTRIB_ID=$(lsb_release -si); export DISTRIB_CODENAME=$(lsb_release -sc)
    #echo "deb [signed-by=/etc/apt/trusted.gpg.d/influxdb.gpg] https://repos.influxdata.com/${DISTRIB_ID,,} ${DISTRIB_CODENAME} stable" | sudo tee /#etc/apt/sources.list.d/influxdb.list > /dev/null

    echo "deb [signed-by=/etc/apt/trusted.gpg.d/influxdb.gpg] https://repos.influxdata.com/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/influxdb.list

    #echo "deb [signed-by=/usr/share/keyrings/influxdb-archive-keyring.gpg] https://repos.influxdata.com/debian $(lsb_release -cs) stable" | sudo #tee /etc/apt/sources.list.d/influxdb.list
    # pull down package list based on this new repo
    sudo apt update

    # install influxdb2
    sudo apt install influxdb2

    # enable start at boot
    sudo systemctl unmask influxdb
    sudo systemctl enable influxdb

Now do setup (also based on https://docs.influxdata.com/influxdb/v2.2/install/?t=CLI+Setup)

    # start the service
    sudo systemctl start influxdb

    # enter CLI setup
    influx setup

## Setup admin user

    influx

Then:

(here <password> = a unique password you save in your password manager)

    CREATE USER admin WITH PASSWORD '<password>' WITH ALL PRIVILEGES
    exit

Edit `influxdb.conf` such that the `[HTTP]` section has:

    auth-enabled = true
    pprof-enabled = true
    pprof-auth-enabled = true
    ping-auth-enabled = true

## Add snitch-agent user / database

Login as admin:

    export INFLUX_DB_ADMIN_PASSWORD=<password>
    influx -username admin -password $INFLUX_DB_ADMIN_PASSWORD

Then:

(here, <snitch-agent-password> = another unique password, different to <password>)

    CREATE DATABASE "snitch-agent";
    CREATE USER "snitch-agent" WITH PASSWORD '<snitch-agent-password>';
    GRANT ALL ON "snitch-agent" to "snitch-agent";
    exit
