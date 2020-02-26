#!/bin/bash

# Exit non-zero if any command in the script exits non-zero
set -e 

# Indicate we are performing maintenance
lucky set-status maintenance 'Running "configure.sh" container script'

# Run a command in the container to get the hostname
hostname=$(hostname)

sleep 2

# Set the status to display the container hostname
lucky set-status active "Container hostname: $hostname"

sleep 2

# Clear the status message
lucky set-status active