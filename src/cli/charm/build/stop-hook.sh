#!/bin/bash

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

# Trigger the `stop` hook
./bin/lucky daemon trigger-hook stop

# Stop the lucky daemon
./bin/lucky daemon stop