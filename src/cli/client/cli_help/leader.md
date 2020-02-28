# Lucky Leader

Interact with the Juju leader system.

${help_message}

## Usage

The `lucky leader` command provides a way to interract with Juju's leader system. You can `get`, `set`, and check whether or not the current user `is-leader`.

The leader unit, elected by Juju, is allowed to set key-value pair using `lucky leader set` and all of the units in the app are able to read key-value pairs from the leader by using `lucky leader get`. See the [Juju documentation](https://discourse.jujucharms.com/t/implementing-leadership/1124) for more information on how to use leadership in Juju.

## Examples

**Set a random password if the unit is the leader:**

```bash
if [ "$(lucky leader is-leader)" = "true" ]; then
    lucky leader set password=$(lucky random)
fi
```

**Get the password from the leader unit:**

```bash
$ lucky leader get password
```

**Set multiple leader values:**

```bash
$ lucky leader set \
    user=username \
    password=topsecret
```
