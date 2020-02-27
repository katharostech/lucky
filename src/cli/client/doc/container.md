# Lucky Container

Control your Lucky containers. 

${help_message}

## Getting Started

The `lucky container` command provides an easy way to run and modify containers as a part of your charm. Most charms will only run one container, but you can run more if desired.

Lucky has the concept of a "default" container, which is the container that is assumed to be operated on by all of the `lucky container` subcommands. Other than the default contianer there can be any number of "named" containers. You can specify that a `lucky container` command should run on a specific named container with the `--container` or `-c` flags.

The first step to setting up a container is to set the container image:

```bash
$ lucky container image set nginx:latest
# Or if you wanted a named container
$ lucky container image set --container frontend nginx:latest
```

You **must** specify the image tag or digest when setting the image. Unlike the Docker CLI, Lucky will not assume you mean to deploy the `latest` tag if you do not specify a tag. If you fail to specify a tag you will get a Docker 404 error in the charm log and the charm script will error.

Once you have set the image for the container, you can use the other `lucky container` subcommands to set different attributes of the container. You can add environment variables, port bindings, and more.

```
# Add a port binding
$ lucky container port add 80:80
# Add an environment variable
$ lucky container env set PASSWORD=topsecret
# Add a volume
$ lucky volume add my-data:/data
```

> **Note:** Not every attribute of containers can be set yet. If you have a need for a container feature that isn't there yet, it is very easy to add new ones, please [create an issue](https://tree.taiga.io/project/zicklag-lucky/issues) and we will look into it.

## How Containers are Run

It is important to understand that the changes to the container configuration made with the `lucky container` subcommands do *not* happen immediately. The changes are applied **after** the current charm script has exited. This allows the charm to make any desired changes to the config and to wait until it is done before making the updates to the container. Lucky is smart about when to apply the Docker updates: it will not do anything if the container configuration after running the script ends up the same as it was before running the script.

If you need to know that your container configuration changes have been applied *before* the script exits you can use the `lucky container apply-updates` command to force Lucky to apply the container config changes.

## Re-deployment and Persistent Data

Whenever a container config update needs to be made, the existing container, if present, will be stopped and removed and a new container will be run with the desired configuration. This means any files changes made in the container will be lost if they are not persisted in a volume. See the [volume](./volume) subcommand for more information on volumes.

## Container Removal

All running containers will be automatically stopped and removed by Lucky when the charm is removed. Currently there is no way to remove containers after creating them. This functionality is on the roadmap. If you need this please [open an issue](https://tree.taiga.io/project/zicklag-lucky/issues).
