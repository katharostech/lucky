# Lucky Client

Interact with Juju, Lucky, and Docker in charm scripts.

${help_message}

## Getting Started

The `lucky client` command contains every command that your charm scripts can use to interact with Juju, Lucky, and Docker.

It is important to realize that this command is *only* used in charm scipts and as help reference for the charm developer. Also, when using this command in charm scripts, you leave out the `client` portion of the command and just use `lucky`. For example, if you locally use `lucky client set-status --help` to find out what options the `set-status` command has, when you use it in your charm scripts, you just put `lucky set-status`, without the `client`.
