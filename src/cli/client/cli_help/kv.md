# Lucky KV

Get and set values in the unit-local key-value store.

${help_message}

## Usage

The `lucky kv` command allows you to interact with the unit's local key-value ( KV ) store. Because this KV store is local to the unit, setting a value in it will not have any effect on the KV store of any other unit in the app cluster. The KV store is a convenient way to maintain any kind of state that the charm might need to keep track of without having to read and write to files or relations.

They KV store will also persist across charm upgrades.

## Examples

**Set a value:**

    $ lucky kv set key=value

**Set multiple values:**

    $ lucky kv set key1=value1 key2=value2 key3=value3

**Get a value:**

    $ lucky kv get key1
    value1

**Get all values:**

    $ lucky kv get
    key1=value1
    key2=value2
    key3=value3

**Delete a value:** Delete values by setting to nothing.

    $ lucky kv set key3=