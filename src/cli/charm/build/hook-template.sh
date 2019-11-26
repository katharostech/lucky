#!/bin/bash
set -e # Exit immediately if a command fails

# The daemon will handle logging so we make these silent to avoid duplicate
# messages.
export LUCKY_LOG_LEVEL=off

# Start the daemon if it is not already running ( it should be, but just in case )
./bin/lucky daemon start --ignore-already-running

# Trigger the `{hook_name}` hook
./bin/lucky daemon trigger-hook {hook_name}