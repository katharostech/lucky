# Lucky - The Lucky charm framwork for Juju

Welcome to the Lucky CLI. Lucky is a framework for creating Juju charms. The Lucky framework is designed to make it ***easy*** and ***fun*** to write charms that are powered by Docker containers.

{{help_message}}

## The Bighelp Pages

Most of the commands in the Lucky CLI have an extra "bighelp" page, like this one, that can be accessed with the `--bighelp` or `-H` flag. We will strive to make these built-in docs sufficient enough to learn to use Lucky without even using the internet!

> **Tip:** On Unix systems or with Windows Git bash you can pipe the docs to `cat` or `less` to skip the pager or use a different one:
> 
> $ lucky -H | less -R # Use less because it is more featured than the built-in pager
> $ lucky -H | cat # Don't use a pager

## Getting Started

The first step to getting started with Lucky is to create your charm using the built-in charm template.

    $ lucky charm create [charmname]

You will be prompted for some basic fields that it will fill out for you in the charm metadata.
