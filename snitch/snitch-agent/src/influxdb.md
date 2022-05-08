# InfluxDB Setup

## Installation

We have to install InfluxDB 1.x instead 2.x because as of 8/5/2022 the only version of InfluxDB that's available on Debian is a 64bit
version and my Raspberry Pi is (currently) 32bit.

### Raspberry Pi

Use https://docs.influxdata.com/influxdb/v1.8/introduction/install/, https://portal.influxdata.com/downloads/ and
https://pimylifeup.com/raspberry-pi-influxdb/ as a base for these instruction as they sets it up via apt
as opposed to a direct .deb install.

    # update all existing software (optional)
    sudo apt update
    sudo apt upgrade

    # get key for signing
    curl https://repos.influxdata.com/influxdb.key | gpg --dearmor | sudo tee /usr/share/keyrings/influxdb-archive-keyring.gpg >/dev/null

    # add the repo
    echo "deb [signed-by=/usr/share/keyrings/influxdb-archive-keyring.gpg] https://repos.influxdata.com/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/influxdb.list

    # pull down package list based on this new repo
    sudo apt update

    # install influxdb
    sudo apt install influxdb

    # enable start at boot
    sudo systemctl unmask influxdb
    sudo systemctl enable influxdb

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

    export INFLUX_DB_ADMIN_PASSWORD='<password>'
    influx -username admin -password $INFLUX_DB_ADMIN_PASSWORD

Then:

(here, <snitch-agent-password> = another unique password, different to <password>)

    CREATE DATABASE "snitch-agent";
    CREATE USER "snitch-agent" WITH PASSWORD '<snitch-agent-password>';
    GRANT ALL ON "snitch-agent" to "snitch-agent";
    exit
