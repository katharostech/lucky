#!/bin/bash
set -e # Exit immediately if a command fails

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

# If log level is set to "trace"
if [ "$(echo $LUCKY_LOG_LEVEL | awk '{{print tolower($0)}}')" = "trace" ]; then
    set -x # Print out bash commands as they are executed
fi

# Replace "/" with "_" in unit name
unit_name=$(echo $JUJU_UNIT_NAME | sed 's/\//_/' )
lucky_data_dir="/var/lib/lucky/$unit_name"
bin_dir="$lucky_data_dir/bin"
log_dir="/var/log/lucky"
mkdir -p $log_dir

# The Lucky executable 
lucky=$bin_dir/lucky

# If Lucky was not bundled
if [ ! -f ./bin/lucky ]; then
    lucky="$bin_dir/lucky"
    # Install the latest Lucky pre-release
    # TODO: Allow specifying a specific version of Lucky to install
    # TODO: Add checks for CPU architecture when downloading
    if [ ! -f $lucky ]; then
        mkdir -p $bin_dir
        curl -L \
            https://github.com/katharostech/lucky/releases/download/pre-release/lucky-linux-x86_64.tgz \
            | tar -xzO > $lucky
    fi
    chmod +x $lucky

# If Lucky was bundled
else
    # Copy lucky binary to the data dir
    if [ ! -f $lucky ]; then
        mkdir -p $bin_dir
        cp ./bin/lucky $lucky
    fi
fi

# Start the Lucky daemon
LUCKY_CONTEXT=daemon $lucky start --ignore-already-running --log-file "$log_dir/$unit_name.log"

# Trigger the `install` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook install