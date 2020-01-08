#!/bin/bash

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

# If Lucky was not bundled
if [ ! -f ./bin/lucky ]; then
    # Replace "/" with "_" in unit name
    unit_dir_name=$(echo $JUJU_UNIT_NAME | sed 's/\//_/' )
    # Use lucky as downloaded by the install script
    lucky="/var/lib/lucky/$unit_dir_name/lucky"
fi

# Trigger the `stop` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook stop

# Stop the lucky daemon
LUCKY_CONTEXT=daemon $lucky stop