# Lucky Charm Build

Build a charm and make it ready for deployment.

{{help_message}}

## Getting Started

The `charm build` command will take a Lucky charm and package it so that it can be deployed to a Juju server or the charm store. By default, the charm will be built to the `build/my_charm_name` directory and can be deployed like so:

    $ juju deploy build/my_charm_name