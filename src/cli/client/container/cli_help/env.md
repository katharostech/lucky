# Lucky Container Env

Get and set container environment variables.

${help_message}

## Examples

**Set a var:**

    $ lucky container env set var=value

**Set multiple vars:**

    $ lucky container env set var1=value1 var2=value2 var3=value3

**Set vars with spaces or newlines in it:**

    $ lucky container env set "var1=value with space
    and newline"

**Get a var:**

    $ lucky container env get var1
    value1

**Get all vars:**

    $ lucky container env get
    var1=value1
    var2=value2
    var3=value3

**Delete a var:** Delete values by setting to nothing.

    $ lucky container env set var3=