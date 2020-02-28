# Lucky Get-Config

Get values from the charm config. 

${help_message}

## Usage

`lucky get-config` is used to get values from the charm configuration. The charm configuration available is defined the the charm's `config.yaml` file. This config allows users to provide input to how the should function.

See the [Juju documentation](https://discourse.jujucharms.com/t/creating-config-yaml-and-configuring-charms/1039) for more information on the `config.yaml` file.

## Examples

**Get the `my-app-version` config:**

    $ lucky get-config my-app-version
    1.33.4

This assumes that there is a `my-app-version` config in the `config.yaml` file, which could look something like this:

**`config.yaml`**:

```yaml
options:
  my-app-version:
    type: string
    default: 1.33.0
    description: The version of My App to install
```