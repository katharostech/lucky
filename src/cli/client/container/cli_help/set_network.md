# Lucky Container Set-Network

Set the docker network to connect the container to.

${help_message}

## Usage

The `lucky container set-network` command is primarily intended to allow you to run the container in host networking mode:

    $ lucky container set-network host

This means that you do not need to bind any ports for the container and that any apps running inside the container will run, from a network perspective, just as they would if they were run on the host.