# Lucky Charm

The Lucky `charm` command contains the tools you will need for charm development.

${help_message}

## Getting Started

The `lucky charm` command contains tools for creating and building your charms. These are the minimal essential tools for Lucky charm developers. You can see the doc pages for the [`create`](./charm/create.md) and [`build`](./charm/build.md) subcommands to learn more.

## Publishing Charms

In order to publish your charms to the charm store you will need the sepparate Juju [charm tools](https://github.com/juju/charm-tools). After you have the charm tools installed, you can push your charm to the charm store:

    # Build the charm using Lucky
    $ lucky charm build
    # Login to the charm store
    $ charm login
    # Push the charm to the charm store
    $ charm push build/charm_name


See the Juju [documentation](https://jaas.ai/docs/charm-writing/store) for more information about publishing charms.