#!/bin/bash
set -e # Exit immediately if a command fails

# Start the lucky daemon
./bin/lucky daemon start

# The daemon will handle logging so we make these silent to avoid duplicate
# messages.
export LUCKY_LOG_LEVEL=off

# Trigger the install hook
./bin/lucky daemon trigger-hook install