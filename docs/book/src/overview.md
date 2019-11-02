# Overview

Lucky is a work-in-progress framework for writing [Juju] charms. It is being designed specifically to support writing Docker-powered charms easily. In the future the framework could be useful for more than Docker charms, but development is currently focused on providing facilities to run and configure Docker containers.

We want Lucky to as easy to use as possible and be very well documented. We will focus on putting the developer's experience first, starting small and adding features as they become necessary or useful.

[juju]: https://jaas.ai/

## Developer Experience

The Lucky framework will provide a charm template that will contain the boilerplate necessary to get started writing a charm with the framework, and it will provide a CLI that will be used by the charm code to interact with Docker and with the Juju controller.

We will be focusing on making it easy to write charms in bash or any other shell language, but, because the framework itself provides a CLI for interacting with it, it is possible to write charm code in Python or any other executable format.

## Development

We are very early in development. We have just started work the proof of concept. We have a [design](./design.md) document outlining our current plan for the framework. This may change as we get into development. If you have any questions or thoughts don't hesitate to open an issue.
