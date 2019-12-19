#!/bin/bash

# Set the lucky log level
export LUCKY_LOG_LEVEL={log_level}

# Trigger the `stop` hook
LUCKY_CONTEXT=daemon ./bin/lucky trigger-hook stop

# Stop the lucky daemon
LUCKY_CONTEXT=daemon ./bin/lucky stop