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

# Grafana setup

Based on https://pimylifeup.com/raspberry-pi-grafana/

    # update all existing software (optional)
    sudo apt update
    sudo apt upgrade

    # get key for signing
    curl https://packages.grafana.com/gpg.key | gpg --dearmor | sudo tee /usr/share/keyrings/grafana-archive-keyrings.gpg >/dev/null

    # add the repo
    echo "deb [signed-by=/usr/share/keyrings/grafana-archive-keyrings.gpg] https://packages.grafana.com/oss/deb stable main" | sudo tee /etc/apt/sources.list.d/grafana.list

    # pull down package list based on this new repo
    sudo apt update

    # install grafana
    sudo apt install grafana

    # enable start at boot
    sudo systemctl daemon-reload
    sudo systemctl enable grafana-server

    # actually start it
    sudo systemctl start grafana-server

Setup admin:

- go to http://<server ip>:3000/login
- log in with `admin` username + `admin` password
- set a new password and save in password manager

## Add dashboard

Based on https://docs.influxdata.com/influxdb/v1.8/tools/grafana/, follow that and the take it from there.

An example query in InfluxQL:

    SELECT mean(download_bandwidth)  * 8 AS download_bandwidth, mean(upload_bandwidth) * 8 AS upload_bandwidth FROM "speedtest" WHERE $timeFilter GROUP BY time($__interval), agent_name fill(null)
