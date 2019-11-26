#!/bin/bash

# The daemon will handle logging so we make these silent to avoid duplicate
# messages.
export LUCKY_LOG_LEVEL=off

# Trigger the `stop` hook
./bin/lucky daemon trigger-hook stop

# Stop the lucky daemon
./bin/lucky daemon stop