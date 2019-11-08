# Lucky - The Lucky charm framwork for Juju

Welcome to the Lucky CLI. Lucky is a framework for creating Juju charms. The Lucky framework is designed to make it ***easy*** and ***fun*** to write charms that are powered by Docker containers.

{{help_message}}

## The Doc Pages and Help

Most of the commands in the Lucky CLI have an extra doc page, like this one, that can be accessed with the `--doc` or `-H` flag. We will strive to make these built-in docs sufficient enough to learn to use Lucky without even using the internet!

Another useful thing to know is that you will get different output by using the `-h` and `--help` flags. The `-h` flag will give you more compact help output while the `--help` flag will give you more details on the available options.

## Getting Started

The first step to getting started with Lucky is to create your charm using the built-in charm template.

    $ lucky charm create [target_directory]

You will be prompted for some basic fields that it will fill out for you in the charm metadata.
