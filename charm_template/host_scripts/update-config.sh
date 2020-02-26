#!/bin/bash

set -e

# This gets the current value of one of the configs from config.yaml
name="$(lucky get-config name)"

lucky set-status maintenance "Config has been changed! Name: $name"

# Set a container environment variable based on the config.
# This will cause the container to be stopped, removed, and re-created after this script exits
# if the container config is not the same as was when the script was started.
lucky container env set "NAME=$name"

sleep 5

lucky set-status active
