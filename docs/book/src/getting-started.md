# Getting Started

Welcome to the getting started guide for the Lucky charming framework. Here we will walk through the process of creating a charm for [CodiMD]. This charm will provide an `http` interface, allowing you to hook it up to other charms such as [HAProxy] for load balancing. The charm will also require a `pgsql` interface which we will use to connect the charm to a [PostgreSQL] database charm.

You can find the entire source for the charm we will be writing in this tutorial [here][charm_source].

[CodiMD]: https://github.com/hackmdio/codimd
[HaProxy]: https://jaas.ai/haproxy
[PostgreSQL]: https://jaas.ai/postgresql
[charm_source]: https://github.com/katharostech/lucky/tree/master/docs/book/src/codimd-example-charm

## Installing Required Tools

Before we get started, we are going to need some tools. Juju development is usually done on Linux and this guide will assume that you are working in a Linux environment. While it is possible to develop charms on Windows, if you already have a Juju cluster, it currently isn't the easiest way to get started.

If your workstation is a Windows machine, the easiest way to develop charms is in a Linux [Vagrant] machine. It is outside of the scope of this tutorial to detail how to setup a Linux Vagrant machine.

> **Note:** If already have a Juju cluster and you have the Juju CLI setup on Windows, you should be able to develop charms on Windows without a VM, but you might require more setup in order to install the charm tools. Even if you don't install the charm tools, you can still develop your charm, you just won't be able to push it to the store.
>
> This is not very tested yet. If you need help getting setup you can open a [forum topic][forum_topic].

[Vagrant]: https://www.vagrantup.com/
[forum_topic]: https://discourse.jujucharms.com/c/related-software/lucky

### Juju

The first step is to setup a Juju development cluster. See the [Juju getting started guide][jgsg] for more information.

[jgsg]: https://jaas.ai/docs/getting-started-with-juju

### Charm Tools

The [charm tools][ct] CLI is what we will use to push our charm to the charm store. It can be installed with its [snap][ct_snap]:

    sudo snap install charm --classic

[ct]: https://github.com/juju/charm-tools
[ct_snap]: https://snapcraft.io/charm

You can skip installing the charm tools if you don't want to push charms to the store.

### Lucky

Now we install Lucky itself. You can download Lucky from the GitHub [releases] page. Lucky will eventually support at least the Snap package manager, but for now you can also use this one-liner to install Lucky:

    curl -L https://github.com/katharostech/lucky/releases/download/v0.1.0-alpha.0/lucky-linux-x86_64.tgz | sudo tar -xzC /usr/local/bin/

You can verify that the install worked:

    $ lucky --version
    lucky v0.1.0-alpha.0

[releases]: https://github.com/katharostech/lucky/releases

## Creating Your Charm

Now that we have the required tools, we can create our charm. Lucky comes with a built-in charm template that you can use:

```bash
$ lucky charm create codimd
Display name [codimd]: Codimd
Charm name [codimd]: 
Charm summary [A short summary of my app.]: A realtime collaborative notes platform.
Charm maintainer [John Doe <johndoe@emailprovider.com>]: My Name <myname@myprovider.com>
```

This will create a new dir named `codimd` with the metadata that you filled in and some example code.

## Configuring Charm Metadata

Lets first take a look at that metadata:

### `metadata.yaml`

```yaml
name: codimd
display-name: Codimd
summary: A realtime collaborative notes platform.
maintainer: My Name <myname@myprovider.com>
description: |
  A realtime collaborative notes platform.
tags:
  # Replace "misc" with one or more whitelisted tags from this list:
  # https://jujucharms.com/docs/stable/authors-charm-metadata
  - misc
subordinate: false
provides:
  provides-relation:
    interface: interface-name
requires:
  requires-relation:
    interface: interface-name
peers:
  peer-relation:
    interface: interface-name
```

That pretty much has what we need, but we will want to change those fake relations to the ones that we actually need. Go ahead and remove the `provides`, `requires`, and `peers` sections and replace them with this:

```yaml
profiles:
  website:
    interface: http
requires:
  database:
    interface: pgsql
```

With this we tell Juju that:

  - we have a `website` relation that we `provide` and the way we interact with that relation conforms to the `http` interface.
  - we have a `database` relation that we `require` and the way we interact with that relation conforms to the `pgsql` interface.

Interfaces are names for the way in which a charm will communicate over a relation. Only relations with the same interface will be allowed to be connected to each-other. This means there is no way to accidentally connect a requires `pgsql` relation to a charm that only provides `http`.

You can find documentation for some interfaces in the [Juju interfaces docs][jid].

Next we'll look at our config.

[jid]: https://discourse.jujucharms.com/c/docs/interfaces

### `config.yaml`

The template config yaml looks like this:

```yaml
# These are config values that users can set from the GUI or the commandline
options:
  name:
    type: string
    default: John
    description: The name of something
  enable-something:
    type: boolean
    default: False
    description: Whether or not to enable something
  count:
    type: int
    default: 100
    description: How much of something
```

The purpose of `config.yaml` is to define the options to our charm that users are allowed to change. We can see all of the available config options for CodiMD in their [documentation][codi_config_doc], but we aren't going to want to add *everything* that is there, and some of it will be configured automatically by our charm, such as the database connection. For now we'll just give some of the minimal essential options in our `config.yaml`.

```yaml
{{#include ./codimd-example-charm/config.yaml}}
```

That config will give us enough information for us to get started, but we would probably want to add the rest of the config later if we were wanting to provide a general purpose charm for the community.

[codi_config_doc]: https://github.com/codimd/server/blob/master/docs/configuration-env-vars.md

### `lucky.yaml`

The final metadata file we are interested in is the `lucky.yaml` file. This file acts as your "control panel" so to speak when it comes to the execution of your charm logic. The job of the `lucky.yaml` is to tell lucky *when* and *how* to execute your charm scripts. The charm template comes with an example `lucky.yaml` that shows you everything that you can put in a `lucky.yaml` file. For the sake of this tutorial we are going to take everything out of the `lucky.yaml` and build on it as we go.

**lucky.yaml:**

```yaml
# Nothing here yet!
```

## Writing Your First Script

Now we are ready to write our first script! In Lucky there are two kinds of scripts, host scripts and container scripts, which are put in the `host_scripts/` and `container_scripts/` directories. The difference is that host scripts run on the host and container scripts are mounted and run inside of the your containers. Our scripts for CodiMD will go in the `host_scripts/` dir.

You will notice that there are some scripts from the charm template already in the `hosts_scripts/` and `container_scripts/` dirs. These are just examples and you can remove them for this tutorial.

The first script that we will create for our charm is the install script. Note that the name of the script is arbitrary and you could call it whatever you want.

**install.sh:**

```bash
{{#include ./codimd-example-charm/host_scripts/install.sh}}
```

First notice that we have a "shabang", as it is called, at the top of the file: `#!/bin/bash`. This tells the system to execute our file with bash. We also include a `set -e`, which will make sure that he script will exit early if any of the commands in it fail. Additionally we need to make our file executable by running `chmod +x install.sh`. This makes sure that Lucky will be able to execute the script when the charm runs.

After that we use the `lucky set-status` command set the status Juju status, which will be visible in the Juju GUI.

Then we set the Docker container image with the `lucky container image set` command. Setting a container's image is the way to create a new contianer that will be deployed by Lucky automatically when our script exits. Additionally, when we change any container configuration, such as environment variables or port bindings, Lucky will wait until our script exits and then apply any changes that we have made. We will see more how this works later.

### Understanding Lucky Status

At this point the Lucky status mechanism should be explained. In Lucky, when you call `set-status`, by default, Lucky will set the given status *for only that script*. This means that if another script uses `set-status` it will not overwrite the status that the previous script set, but will instead *add* its status to the previous status by comma separating the list of all script status messages.

It is common pattern in Lucky scripts to have a `lucky set status maintenance "Message"` at the top  of the script and a `lucky set-status active` at the bottom. This makes sure that the user will be notified of the scripts action, and that the action message will be cleared before the script exits.

Alternatively, when you set a status with a `--name <name>`, you can set that status from *any* script by specifying the same `--name`. In this exmple, we use a status with a `db-state` name that we use to indicate the status of our database connection. When the charm is first installed, it will not have a database relation, and we use this opportunity to tell the user that we need a database connection to work. Later, when we get a database connection in a *different* script, we will call `lucky set-status --name db-state active` to clear the blocked status.

### Adding Our Script to the `lucky.yaml`

Ok, so we now have a written script, but currently there is nothing instructing Lucky to run the script at any time. The script existing is *not* enough to cause it to run. That is why we add entries to the `lucky.yaml`, to tell Lucky when to run our scripts.

In this case, we want our `install.sh` host script to run when the Juju `install` hook is triggered:

**lucky.yaml:**

```yaml
{{#include ./codimd-example-charm/lucky.yaml:1:6}}
```

Pretty simple right? Now Lucky will run our `install.sh` host script whenever the Juju `install` hook is run.

Lets move on to the `configure.sh` script.

## Writing the `configure.sh` Script

So we have our app installing, and actually starting ( becuse Lucky will start the container when we set the docker image ) with the `install.sh` script, but it won't really do anything because it doesn't have any of our configuration. That is what we are going to do with our `configure.sh` script. We are going to read the configuration values that we have defined in our `config.yaml` and use those values to set environment variables on our CodiMD container.

**configure.sh:**

```bash
{{#include ./codimd-example-charm/host_scripts/configure.sh}}
```

In this script we introduce some extra `lucky` commands. As always, you can access extra information on those commands in the [Lucky client](./cli/lucky/client.md) documentation.

> **Note:** You can also access the CLI documentation from the Lucky CLI itself, by prefixing the command that we use in our script with `client` and adding the `--doc` flag. For example, you can run `lucky client get-config --doc` on your workstation to get terminal rendered view of the same CLI documentation available on this site. This can be very useful when needing to quickly look something up without using a web browser or the internet.

Overall this script is pretty simple, when the config changes, we make sure that our container environment variables are up-to-date. Also we make sure that we mount the configured port on the host to the container. When working with ports that are opened according to configuration values, we need to make sure that we remove any ports that were opened by previous configuration. This makes sure that we don't end up with multiple ports mounted into the container if the user changes the configured port and the `configure.sh` script is re-run.

### The Difference Between `lucky container port` and `lucky port`

You may notice in the above example that we do both a `lucky container port add` and a `lucky port open`, so what is the difference?

`lucky contianer port add` will add a port binding from the host to the container. `lucky port open`, on the other hand, registers that port with Juju so that it will be opened through the hosts *firewall* when users run `juju expose codimd`.

If you want to be able to communicate to a port only on the private network, such as app-to-app communication, you do **not** want to use `lucky open` because that will expose that port to the internet on the hosts firewall. In such a case you will still need to use `lucky contianer port add` to make sure that the containers can communicate.

If you *do* want to be able to hit the port from the internet, though, like in the case of CodiMD, you *will* need to `lucky open port` *and* the users will need to `juju expose` the application before you can access that port.

### Adding `configure.sh` to the `lucky.yaml`

Now we can add `configure.sh` to the `lucky.yaml` just like we did with the `install.sh` script.

```yaml
{{#include ./codimd-example-charm/lucky.yaml:1:10}}
```

## Handling the DB Relation