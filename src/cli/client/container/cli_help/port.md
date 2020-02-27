# Lucky Container Port

Add, remove, and list container port bindings.

${help_message}

## Usage

`lucky container port` allows you to control which ports in the container get bound to which ports on the host.

It is important to understand that adding port bindings with `lucky container port add` will *append* the port binding to any existing port bindings. If you want to make sure that the contianer *only* has the bindings that you specify at a particular moment in time you must first run `lucky container port remove --all`.

## Examples

**Bind port 80 on the host to 80 in the container:**

    $ lucky container port add 80:80

**Bind port 8080 on the host to 80 in the container:**

    $ lucky container port add 8080:80

**Remove the port binding of 80 on the host to 80 in the contianer:**

    $ lucky container port remove 

**Get a list of the port bindings for the container:**

    $ lucky container port list
    80:80
    443:443

**Make sure the *only* port bindings on the container are `80:80` and `443:443` :**

    $ lucky container port remove --all
    $ lucky container port add 80:80
    $ lucky container port add 443:443