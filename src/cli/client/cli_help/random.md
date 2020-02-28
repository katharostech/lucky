# Lucky Random

Generate random passwords, numbers, and available ports.

${help_message}

## Usage

`lucky random` is a utility for generating passwords, number ranges, and for getting random available ports.

## Examples

**Generate a random password with a specified length:**

    $ lucky random --length 32

**Generate a random integer between 0 and 10 inclusive:**

    $ lucky random --range 0 10

**Generate a random float between 0 and 1:**

    $ lucky random --float --range 0 1

**Find a random available port in between `1024` and `65535`:**

    $ lucky random --available-port
