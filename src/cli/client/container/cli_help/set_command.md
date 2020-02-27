# Lucky Container Set-command

Set the containers `command`.

${help_message}

## Examples

The command args must come after a `--` as shown in these examples:

**Set command to `echo hello`:**

    $ lucky container set-command -- echo hello

**Set command to `example --arg`:**

    $ lucky container set-command -- example --arg

**Set the container entry point and command so that it will `tail -f /dev/null`:**

    $ lucky container set-entrypoint tail
    $ lucky container set-command -- -f /dev/null