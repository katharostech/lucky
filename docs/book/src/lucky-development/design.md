# Design

The Lucky framework will be implemented in [Rust] and will consist of a daemon that runs as a part of the charm and a CLI that will be used by scripts to communicate to the daemon. The overall design can be seen in the diagram below.

![charm-framework-diagram](./assets/lucky-framework.svg)

[rust]: https://www.rust-lang.org/

## Hooks

Just like every Juju charm, Lucky charms implement a number of different hooks that the Juju controller will execute. These hooks will not be implemented by the developer but will be provided by the Lucky charm template.

### Install

The install hook will first download one of our automated builds of the Lucky framework, which will be a standalone Rust executable. The install hook will be sure to download the binary appropriate to the platform architecture.

After downloading the Lucky binary it will run the Lucky daemon.

> **Note:** The Lucky binary also acts as the CLI that is used to communicate with the running daemon.

### Other Hooks

All of the other hooks are scripts that simply use the Lucky CLI to notify the Lucky daemon of the hook's execution and of the environment variables that came with the hook execution. It is then the Lucky daemon's job to trigger the appropriate user scripts.

## The Lucky Daemon and CLI

The Lucky daemon will be run by the charm install hook and will continue running for the whole duration that the charm is installed. The daemon will listen on a Unix socket for commands that will be sent to it by the Lucky CLI.

As noted above, the daemon will be notified for every Juju hook that is executed. It is the daemon's job to, in response to the hooks, trigger the charm developer's own scripts and to be those scripts' interface to both Docker and the Juju hook environment.

When a developer writes scripts for Lucky charms, instead of using the normal Juju commands such as `set-status` and `get-relation` provided by the Juju hook environment, the scripts will use the Lucky CLI. The reason for this is that scripts executed inside of the Docker container will not have access to the Juju hook environment. By mounting the Lucky daemon's socket and the Lucky CLI into the Docker container, we provide a way for scripts inside of the container to interact with the Juju hook environment. The Lucky daemon will also set helpful environment variables that can be used by the scripts, including the ones that exist in the Juju hook environment.

The CLI will also provide commands to configure how the container is run, such as adding environment variables, ports, starting/stopping the container, etc. The charm scripts will not themselves execute Docker commands directly, but will use the Lucky CLI instead.

## Charm Scripts

The charm developer will write two kinds of scripts, host scripts and container scripts. Both kinds of are essentially similar to the normal Juju hooks and can be any executable format. All of a charms container scripts will be mounted into the Docker container and will execute inside of the container while all of the host scripts will be run on the host. The scripts will be executed by the Lucky daemon in response to the Juju hooks as outlined in a YAML file that could look something like this:

**lucky.yml:**
```yaml
hooks:
  install:
    - host_script: do-some-host-stuff.sh
    - container_script: do-some-stuff-in-container.sh
  update-status:
    - host_script: health-check-service-from-host.sh
    - container_script: make-sure-process-in-container-is-running.py
  config-changed:
    - container_script: update-application-config.sh
```

The scripts will be executed by the Lucky daemon in the order that they are defined in the YAML.
