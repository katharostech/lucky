# Lucky - The Lucky charm framwork for Juju

Welcome to the Lucky CLI. Lucky is a framework for creating Juju charms. The Lucky framework is designed to make it ***easy*** and ***fun*** to write charms that are powered by Docker containers.

{{help_message}}

## Development

If you are reading this now, you probably know that Lucky is under heavy development and is currently incomplete. This will hopefully change soon! Please pardon the missing features and documentation as we work to get out our first release.

If you have any thoughts don't be afraid to leave feedback on GitHub ( *https://github.com/katharostech/lucky* ) or ping/private message @zicklag on the Juju forum ( *https://discourse.jujucharms.com* ).

## The Doc Pages and Help

Most of the commands in the Lucky CLI have an extra doc page, like this one, that can be accessed with the `--doc` or `-H` flag. We will strive to make these built-in docs sufficient enough to learn to use Lucky without even using the internet!

Another useful thing to know is that you will get different output by using the `-h` and `--help` flags. The `-h` flag will give you more compact help output while the `--help` flag will give you more details on the available options.

## Getting Started

The first step to getting started with Lucky is to create your charm using the built-in charm template.

    $ lucky charm create [target_directory]

You will be prompted for some basic fields that it will fill out for you in the charm metadata. The doc page for `lucky charm create` has more information about what the different files in the charm are for.

Once you have tweak your new charm and added your own scripts you must build the charm. Building the charm packages it so that it can be deployed to a Juju server or the charm store.

    $ lucky charm build

That should complete almost immediately and you can then deploy the charm to a Juju controller:

    $ juju deploy build/charm_name
