# Lucky Public-Address

Get the unit's public address.

${help_message}

## Usage

`lucky public-addresss` will return the public address of the current unit. The server should be accessible from the internet by the public address. Before users can connect to your app from the internet, though, you will need to open ports using `lucky port open` and the user of the charm will need to expose the app with `juju expose my-app`.