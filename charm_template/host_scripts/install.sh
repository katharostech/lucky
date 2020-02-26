#!/bin/bash

# NOTE: The "shabang" above is required for the script to execute properly

# Exit non-zero is any command in the script exits non-zero
set -e

# Say that we are in the middle of installing
lucky set-status maintenance 'Running "install.sh" host script.'

# Pretend to install something
sleep 10

# Create a Docker container by setting the container image to use
lucky container image set nginx:latest # Be sure to include the tag

# Bind the ningx port
lucky container port add 80:80

# Open the port so that it will be exposed through the firewall if the user runs
# `juju expose`
lucky port open 80

# The Docker container will be run with all the settings we just set when this script exits

# Indicate we are done installing
lucky set-status active
