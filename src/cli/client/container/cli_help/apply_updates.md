# Lucky Container Apply-Updates

Apply changes to docker configuration in the middle of a charm script.

${help_message}

## Usage

The `lucky container apply-updates` command is used to apply any changes that have been made to the container configuration before the current script has exited. Normally Lucky will wait until your script has exited before it applies the container configuration, but this gives you a way to make sure that the updates have applied before executing further logic.

## Examples

```bash
# Change an environment variable
lucky container env set PASSWORD=topsecret

# Apply the container configuration
lucky container apply-updates

# Continue doing stuff that depend on the container `PASSWORD` having been updated
```