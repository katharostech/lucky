#!/bin/bash
set -e # Exit immediately if a command fails

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
log_dir="/var/log/lucky"
mkdir -p $log_dir
unit_state_dir="/var/lib/lucky/$unit_name"

# If Lucky was not bundled
if [ ! -f ./bin/lucky ]; then
    # Use lucky as downloaded by the install script
    lucky="/var/lib/lucky/$unit_name/bin/lucky"
fi

# Start the Lucky daemon
LUCKY_CONTEXT=daemon $lucky start --ignore-already-running --log-file "$log_dir/$unit_name.log"

# Trigger the `{hook_name}` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook {hook_name}