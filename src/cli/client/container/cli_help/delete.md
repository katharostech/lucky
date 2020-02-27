# Lucky Container Delete

Delete a container.

${help_message}

## Usage

`lucky container delete` will remove a container. Like all `lucky container` commands, you can specify which container to delete with the `--container` flag. Also like the other `container` commands, the container will be deleted after the current script exits unless you force the update with `lucky container apply-updates`.

## Examples

```bash
# Delete the default container
lucky container delete
```