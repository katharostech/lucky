#!/bin/bash

# The daemon will handle logging so we make these silent to avoid duplicate
# messages.
export LUCKY_LOG_LEVEL=off
./bin/lucky daemon trigger-hook stop
./bin/lucky daemon stop