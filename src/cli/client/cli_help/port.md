# Lucky Port

Open and close ports on the host firewall.

${help_message}

## Usage

The `lucky port` command allows you to `open`, `close`, and `get-opened` ports on the host firewall.

> **Note:** Juju will not actually expose the opened firewall ports unless the user deploying the charm exposes it using `juju expose my-app`. In this way, apps are not accidentally exposed to the internet without the user meaning to.

## Examples

**Make sure your configured port is the only port opened:**

```bash
# Make sure any previously opened ports are closed
$ lucky port close --all
# Open the new ports
$ lucky port open $(lucky get-config listen-port)
```