# Lucky Charm Build

Build a charm and make it ready for deployment.

${help_message}

## Getting Started

The `lucky charm build` command will take a Lucky charm and package it so that it can be deployed to a Juju server or the charm store. By default, the charm will be built to the `build/my-charm` directory and can be deployed like so:

```bash
$ lucky charm build
$ juju deploy ./build/my-charm
```

## Building With Local Lucky

When building with the `--use-local-lucky` or `-l` argument, Lucky will bundle the local version of Lucky that was used to build the charm into the built charm. This means that the charm will not attempt to download Lucky when it starts up and that the charm will only run on the same CPU architecture. This is mostly useful during development and only works on Linux builds made with the "daemon" feature.

If this is not specified, an automated build of Lucky for the architecture that the charm is deployed to will be automatically downloaded when the charm is installed.