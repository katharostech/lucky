# Overview

Lucky is a work-in-progress framework for writing [Juju] charms. It is being designed specifically to support writing Docker-powered charms easily. In the future the framework could be useful for more than Docker charms, but development is currently focused on providing facilities to run and configure Docker containers.

We want Lucky to be as easy to use as possible and be very well documented. We will focus on putting the developer's experience first, starting small and adding features as they become necessary or useful.

[juju]: https://jaas.ai/

## Developer Experience

The Lucky framework will provide a CLI that will be installed on the developers workstation that will be used to create and build Lucky charms. We will be focusing on making it easy to write charms in bash or any other shell language, but, through the Lucky commandline interface, any programming language could be used.

## Development

We are very early in development. We have just started work on the proof of concept. We have a [design](./design.md) document outlining our current plan for the framework. This may change as we get into development. If you have any questions or thoughts don't hesitate to [contact us](#bugs-features-and-questions).

## Bugs, Features, and Questions

If you have any bug reports or feature requests you can create a [Taiga issue][taiga_issue] and we'll see what we can do to help.

For questions or general discussion there is also a [Lucky category][category] on the [Juju forum][juju_forum].

[category]: https://discourse.jujucharms.com/c/related-software/lucky
[juju_forum]: https://discourse.jujucharms.com
[taiga_issue]: https://tree.taiga.io/project/zicklag-lucky/issues