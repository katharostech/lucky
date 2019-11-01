# Design

The {toolname} framework will be implemented in Rust and will consist of a daemon that runs as a part of the charm and a CLI that will be used by scripts to communicate to the daemon. The overall design can be seen in the diagram below.

![charm-framework-diagram](./assets/charm-framework.svg)

To explain how the design works we will go through the different components step by step.

## Hooks

Just like every Juju charm, charms built with the {toolname} framework implement a number of different hooks that the Juju controller will execute. These hooks will not be implemented by the developer but will be provided by the {tollname} charm template.

### Install

The install hook will first download one of our automated builds of the {toolname} framework, which will be a standalone Rust executable. The install hook will be sure to download the binary appropriate to the platform architecture.

After downloading the {toolname} binary it will run the {toolname} daemon. The {toolname} binary also acts as the CLI that is used to communicate with the running daemon. The install hook will use this CLI to tell the daemon to execute the developer's install hooks. This will be explained in more detail later.

### Other Hooks

All of the other hooks are scripts that simply use the {toolname} CLI to tell the {toolname} daemon that it needs to execute the code related to the triggered hook.

> **Note:** On a somewhat related note, in the event that something goes wrong that somehow kills the daemon process, when the next hook is triggered by Juju, the CLI will detect that the daemon has stopped and will start it again before notifying the daemon of the hook.

## The {toolname} Daemon

The {toolname} daemon will be run by the charm and will continue running for the whole duration that the charm is installed. The daemon will listen on a Unix socket for commands that will be sent to it by the {toolname} CLI. The {toolname} daemon and CLI are provided by the same binary.
