#!/bin/bash

set -e

lucky set-status maintenance "Configuring CodiMD"

# Get the config values and put them in shell variables
domain="$(lucky get-config domain)"
url_path="$(lucky get-config url-path)"
https="$(lucky get-config https)"
port="$(lucky get-config port)"

# Set the respective container environment variables
lucky container env set \
    "CMD_DOMAIN=$domain" \
    "CMD_URL_PATH=$url_path" \
    "CMD_PORT=$port" \
    "CMD_PROTOCOL_USESSL=$https"

# Remove any mounted container ports that might have been added in previous
# runs of `configure.sh`.
lucky container port remove --all

# Mount configured port on the host to configured port in the container
lucky container port add "$port:$port"

# Clear any previously opened firewall ports
lucky port close --all

# Open the configured port on the firewall
lucky port open $port

lucky set-status active