# Lucky Charm Create

Create a new Lucky charm from a template.

${help_message}

## Getting Started

Running `lucky charm create` is the first step to getting started writing a Lucky charm. The command will prompt you for some basic information about your new charm and will then create all of the files necessary to get started.

## Files

Here are the files you will need to modify to get started on your charm.

### `metadata.yaml`

The `metadata.yaml` file has information about the charm such as name, description, and the kind of relations it supports. The [Juju Documentation](https://discourse.jujucharms.com/t/charm-metadata/1043) has more information on what can go in the `metadata.yaml`.

### `config.yaml`

The `config.yaml` file outlines the different configuration options that users can provide to the charm. This might be used for things like admin passwords, web server ports, or other similar options. More information on charm config can be found in the [Juju Config](https://discourse.jujucharms.com/t/creating-config-yaml-and-configuring-charms/1039).

### `lucky.yaml`

The `lucky.yaml` file is your "control panel" so to speak for what charm code gets executed when, in response to things like [Juju hooks](https://discourse.jujucharms.com/t/charm-hooks/1040) or cron jobs. Without telling Lucky to execute your charm scripts by adding entries to the `lucky.yaml`, your charm will not do anything.

The example `lucky.yaml` file that comes with the charm template has commented sections indicating all of the different kinds of entries you can add to the `lucky.yaml`.

### `host_scripts/` and `container_scripts/`

These directories contiain your charm scripts which represent the charm's logic. Each of the files in these dirs should be executable and are usually shell scripts starting with a proper "shabang": `#!/bin/bash`. The `host_scripts` dir contains scripts that are to be executed on the host system while `container_scripts` contains scripts that will be mounted into any containers that the charm runs.

These scripts will use the `lucky client` commands to interact with Juju, Lucky, and Docker. Se the `lucky client` doc page for more information.