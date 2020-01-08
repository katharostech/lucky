#!/bin/bash
set -e # Exit immediately if a command fails

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

# The Lucky executable 
lucky=./bin/lucky

# If Lucky was not bundled
if [ ! -f ./bin/lucky ]; then
    # Replace "/" with "_" in unit name
    unit_dir_name=$(echo $JUJU_UNIT_NAME | sed 's/\//_/' )
    unit_state_dir="/var/lib/lucky/$unit_dir_name"
    lucky="$unit_state_dir/lucky"

    # Install the latest Lucky pre-release
    # TODO: Allow specifying a specific version of Lucky to install
    # TODO: Add checks for CPU architecture when downloading
    mkdir -p $unit_state_dir
    curl -L \
        https://github.com/katharostech/lucky/releases/download/pre-release/lucky-linux-x86_64.tgz \
        | tar -xzO > $lucky
    chmod +x $lucky
fi

# Start the Lucky daemon
LUCKY_CONTEXT=daemon $lucky start --ignore-already-running

# Trigger the `install` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook install