#!/bin/bash

# Exit script early if any command in the script fails
set -e

# Set the status for this script so users can se what our charm is doing
lucky set-status maintenance "Starting CodiMD"

# Set the Docker image, this will cause lucky to create a container when this
# script exits
lucky container image set quay.io/codimd/server:1.6.0-alpine
lucky set-status --name db-state blocked "Waiting for database connection" 

# Set a named status that can be changed from other scripts.
# Here we notify the user that we need a database relation before CodiMD will work

# Clear the status for this script by setting the status to active without a message.
# This makes sure that our "Starting CodiMD" message goes away.
lucky set-status active