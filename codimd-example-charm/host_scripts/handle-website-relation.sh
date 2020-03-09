#!/bin/bash

set -e

# Here we match on the first argument ( $1 ) passed in from the lucky.yaml

# If we are joining a new relation
if [ "$1" = "join" ]; then
    lucky set-status maintenance "Joining HTTP relation"

    # Set hostname and port values for the joined relation
    lucky relation set \
        "hostname=$(lucky private-address)" \
        "port=$(lucky get-config port)"

# If we are supposed to update our existing relations
elif [ "$1" = "update" ]; then
    lucky set-status maintenance "Updating HTTP relations"

    # For every appliacation connected to or "website" relation
    for relation_id in $(lucky relation list-ids --relation-name website); do
        # Set the hostname and port values for the relation
        lucky relation set --relation-id $relation_id \
            "hostname=$(lucky private-address)" \
            "port=$(lucky get-config port)"
    done
fi

lucky set-status active