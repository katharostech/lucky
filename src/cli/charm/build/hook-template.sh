#!/bin/bash
set -e # Exit immediately if a command fails

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

if [ ! -f ./bin/lucky ]; then
    # TODO: Download or install Lucky
    echo "TODO: Download or install Lucky"
fi

# Start the Lucky daemon
LUCKY_CONTEXT=daemon ./bin/lucky start --ignore-already-running

# Trigger the `{hook_name}` hook
LUCKY_CONTEXT=daemon ./bin/lucky trigger-hook {hook_name}