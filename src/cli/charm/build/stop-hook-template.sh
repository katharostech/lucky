#!/bin/bash

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

# If log level is set to "trace"
if [ "$(echo $LUCKY_LOG_LEVEL | awk '{{print tolower($0)}}')" = "trace" ]; then
    set -x # Print out bash commands as they are executed
fi

# The Lucky executable
lucky=./bin/lucky

# Replace "/" with "_" in unit name
unit_name=$(echo $JUJU_UNIT_NAME | sed 's/\//_/' )
lucky_data_dir="/var/lib/lucky/$unit_name"

# If Lucky was not bundled
if [ ! -f ./bin/lucky ]; then
    # Use lucky as downloaded by the install script
    lucky="$lucky_data_dir/bin/lucky"
fi

# Trigger the `stop` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook stop

# Stop the lucky daemon
LUCKY_CONTEXT=daemon $lucky stop

# Clean up the charm bin dir
rm -rf "$lucky_data_dir/bin"