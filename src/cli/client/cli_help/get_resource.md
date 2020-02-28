# Lucky Get-Resource

Get the path to a Juju resource. 

${help_message}

## Usage

This command is the direct equivalent to Juju's [`resource-get`](https://discourse.jujucharms.com/t/using-resources-developer-guide/1127#heading--using-resources-in-a-charm) command. Refer to that documentation for more information on how Juju resources work.

This command will exit non-zero if the resource has not been uploaded yet.

## Example

**Make sure a resource exists before getting the path to it:**

```bash
if ! lucky get-resource resource-name; then
    lucky set-status blocked "Need resource-name to be uploaded"
else
    path_to_resource=$(lucky get-resource resource-name)
fi
```