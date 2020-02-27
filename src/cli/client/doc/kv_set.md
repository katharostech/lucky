# Lucky KV Set

Set values in the unit-local key-value store.

${help_message}

## Examples

**Set a value:**

    $ lucky kv set key=value

**Set multiple values:**

    $ lucky kv set key1=value1 key2=value2 key3=value3

**Set multiple values across multiple liines:**

    $ lucky kv set \
        key1=value1 \
        key2=value2 \
        key3=value3

**Set values with spaces or newlines:**

    $ lucky kv set "key=value with spaces
    and newlines"