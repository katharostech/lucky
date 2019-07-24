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

# If Lucky was not bundled
if [ ! -f ./bin/lucky ]; then
    # Use lucky as downloaded by the install script
    lucky="/var/lib/lucky/$unit_name/bin/lucky"
fi

# Trigger the `stop` hook
LUCKY_CONTEXT=daemon $lucky trigger-hook stop

# Stop the lucky daemon
LUCKY_CONTEXT=daemon $lucky stop

# Clean up the lucky state and bin dirs 
# ( leave the parent dir there in case there is any docker volume data in it )
rm -rf "/var/lib/lucky/$unit_name/state"
rm -rf "/var/lib/lucky/$unit_name/bin"