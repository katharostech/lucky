# Lucky - The Lucky charm framwork for Juju

Welcome to the Lucky CLI. Lucky is a framework for creating Juju charms. The Lucky framework is designed to make it ***easy*** and ***fun*** to write charms that are powered by Docker containers.

${help_message}

## Development

Lucky is currently under active development and is in an alpha state. Features and documentation may be missing, but we at Katharos Technology are already producing charms with Lucky that are being used in the wild. Lucky is getting real testing and solving real problems.

If you have any thoughts or questions please don't hesitate to [open a forum topic](https://discourse.jujucharms.com/new-topic) in the Lucky category on the Juju forum. You can also make [feature requests or bug reports](https://tree.taiga.io/project/zicklag-lucky/issues) on our Taiga instance.

## The Doc Pages and Help

Most of the commands in the Lucky CLI have an extra doc page, like this one, that can be accessed with the `--doc` or `-H` flag. These will usually have extra information and examples on how to use the command.

Another useful thing to know is that you will get different output by using the `-h` and `--help` flags. The `-h` flag will give you more compact help output while the `--help` flag will give you more details on the available options.

## Getting Started

The first step to getting started with Lucky is to create your charm using the built-in charm template.

    $ lucky charm create my-first-charm

You will be prompted for some basic fields that it will use to fill in the charm metadata. The doc page for `lucky charm create` has more information about what the different files in the charm are for.

After you have created your charm, you need to edit the `lucky.yaml` file to configure which scripts to run as a part of your charm. The charm template comes with some scripts and an example `lucky.yaml` file with comments that show the available options.

Once you have uncommented out the lines in the `lucky.yaml` you have to build the charm. Building the charm packages it so that it can be deployed to a Juju server or the charm store.

    $ cd my-lucky-charm
    $ lucky charm build

The build should complete almost immediately and you can then deploy the charm to a Juju controller:

    $ juju deploy ./build/my-lucky-charm

## Learning More

For a full tutorial you can read the [Getting Started guide](https://katharostech.github.io/lucky/development.html) in the Lucky documentation.