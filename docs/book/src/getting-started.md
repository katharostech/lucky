# Getting Started

Welcome to the getting started guide for the Lucky charming framework. Here we will walk through the process of creating a charm for [CodiMD]. This charm will provide an `http` interface, allowing you to hook it up to other charms such as [HAProxy] for load balancing. The charm will also require a `pgsql` interface which we will use to connect the charm to a [PostgreSQL] database charm.

[CodiMD]: https://github.com/hackmdio/codimd
[HaProxy]: https://jaas.ai/haproxy
[PostgreSQL]: https://jaas.ai/postgresql


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
options:
  domain:
    type: string
    default: example.org
    description: The domain CodiMD is hosted under.
  url-path:
    type: string
    default: ""
    description: If CodiMD is run from a subdirectory like "www.example.com/<urlpath>"
  port:
    type: string
    default: RANDOM
    description: The port to host CodiMD on, or "RANDOM" to have the charm pick a random port.
  https:
    type: boolean
    default: false
    description: Whether or not the server will be accessed over HTTPS
```

That config will give us enough information for us to get started, but we would probably want to add the rest of the config later if we were wanting to provide a general purpose charm for the community.

[codi_config_doc]: https://github.com/codimd/server/blob/master/docs/configuration-env-vars.md