# Lucky Relation

Communicate over Juju relations. 

${help_message}

## Understanding Relations

The `lucky relation` command has all of the tools that you need to interact with Juju's concept of relations. Relations are a somwhat complicated concept to grasp but they are probably the most important concept in Juju and they are what powers Juju's ability to deploy apps in a conceptually simple manner.

Juju relations allow the units on each side of a relation to *set* values on their side of the relation that can be *read* by the unit on the other side. A unit can `set` and `get` values on its own side of the relation, but it can only `get` values from the other side of the relation.

While we hope to provide more documentation on using relations later, currently the best reference is the [Juju documentation](https://discourse.jujucharms.com/t/the-hook-environment-hook-tools-and-how-hooks-are-run/1047#heading--relation-get). The lucky relation commands serve the same functions as the built-in Juju hook tools. The mappings are as follows:

|:-:|:-:|
|**Lucky Command**|**Juju Command**
|:-|:-|
|`lucky relation set`|`relation-set`|
|`lucky relation get`|`relation-get`|
|`lucky relation list-ids`|`relation-ids`|
|`lucky relation list-units`|`relation-list`|
|-

## Examples

**Iterating over charm relations:**

This will work inside or outside of a `relation-*` hook.

```bash
servers=""
# For each related application in the `http` relation
for relation_id in $(lucky relation list-ids --relation-name http); do
    
    # For every unit of that application
    for related_unit in $(lucky relation list-units -r $relation_id); do
        
        # Get the unit's hostname
        addr=$(lucky relation get -r $relation_id -u $related_unit hostname)
        
        # Get the unit's port
        port=$(lucky relation get -r $relation_id -u $related_unit port)
        
        # Add it to the server list
        servers="$servers $addr:$port"

        # You can also set values on your side of the relation
        lucky relation set -r $relation_id -u $related_unit key=value
    done

    # You can also get the values that you have set on this relation by specifying your unit name
    # ( from the $JUJU_UNIT_NAME env var ) as the -u argument
    lucky relation get -r $relation_id -u $JUJU_UNIT_NAME hostname
done
```

---

**Get the hostname and port of a related unit in a `http-relation-changed` hook:**

In this example, because we are in a `http-relation-changed` hook, we don't have to specify the relation id or the related unit. Those are set in environment variables by Juju and the `lucky relation get` command will automatically default to getting from the relation and unit that triggered the `relation-changed` hook.

```bash
hostname=$(lucky relation get hostname)
port=$(lucky relation get port)
```

You can also set data on relations in hooks in the same way.

```bash
lucky relation set user=my-username
```

