# Lucky Container Set-Entrypoint

Set the containers `entrypoint`.

${help_message}

## Example

**Set the container entry point and command so that it will `tail -f /dev/null`:**

```bash
$ lucky container set-entrypoint tail
# Because command arg starts with a `-` it must come after a lone `-` arg
$ lucky container set-command -- -f /dev/null
```