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
    # Use lucky as downloaded by the install script
    lucky="/var/lib/lucky/$unit_dir_name/bin/lucky"
fi

# Start the Lucky daemon
LUCKY_CONTEXT=daemon $lucky start --ignore-already-running

# Trigger the `{hook_name}` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook {hook_name}