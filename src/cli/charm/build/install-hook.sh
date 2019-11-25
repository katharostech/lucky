#!/bin/bash
set -e # Exit immediately if a command fails
./bin/lucky daemon start
./bin/lucky daemon trigger-hook install