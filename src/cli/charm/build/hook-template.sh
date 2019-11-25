#!/bin/bash
set -e # Exit immediately if a command fails
./bin/lucky daemon start --ignore-already-running
./bin/lucky daemon trigger-hook {hook_name}