#!/bin/bash

# Trigger the `stop` hook
./bin/lucky daemon trigger-hook stop

# Stop the lucky daemon
./bin/lucky daemon stop