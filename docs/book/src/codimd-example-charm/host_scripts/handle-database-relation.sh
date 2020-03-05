#!/bin/bash

set -e

# Set the database name
db_name=codimd

# Here we match on the first argument ( $1 ) passed in from the lucky.yaml

if [ "$1" = "join" ]; then
    lucky set-status --name db-state maintenance "Connecting to database"

    # Set the name of the database that we want the server to create for us
    lucky relation set "database=$db_name"

elif [ "$1" = "update" ]; then
    lucky set-status --name db-state maintenance "Updating database connection"

    # Get the values from the connected database relation
    dbhost="$(lucky relation get host)"
    dbport="$(lucky relation get port)"
    dbuser="$(lucky relation get user)"
    dbpassword="$(lucky relation get password)"

    # If any of those values have not be set yet, exit early and wait until next update
    if [ "$dbhost" = "" -o "$dbport" = "" -o "$dbuser" = "" -o "$dbpassword" = "" ]; then
        exit 0
    fi

    # Set database connection environment variable
    lucky container env set "CMD_DB_URL=postgres://$dbuser:$dbpassword@$dbhost:$dbport/$db_name"

    lucky set-status --name db-state active

elif [ "$1" = "leave" ]; then
    lucky set-status --name db-state maintenance "Disconnecting from database"

    # Unset database connection environment variable
    lucky container env set "CMD_DB_URL="

    lucky set-status --name db-state blocked "Waiting for database connection"
fi