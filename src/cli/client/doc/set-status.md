# Lucky Set-Status

Set the Juju status of the script.

${help_message}

## Usage

`lucky set-status`, by default, will set the status **for the script *and* hook combination that it is run in**. This means that running `lucky set-status` in one script, will not overwrite the satus set by another script. Also, if you set the status in a script that was triggered by one hook, it will not overwrite the status set by the same script in a different hook.

When multiple status are set at the same time, the Juju status will be set to a comma separated list of the statuses. This means that, when multiple scripts are running at the same time, you will not have to worry about their statuses getting overwritten by other scripts that are also trying to set the status.

It is typical in charm scripts do something like this:

```bash
# At the beginning of the script
lucky set-status maintenance "Doing something"

# Do stuff...

# At the end of the script
lucky set-status active
```

This will show that you are "Doing something" while the script is running, and then clear the status at the end of the script.

## Setting the Status Name

When setting the status, you can specify the `--name` or `-n` flag to set a specific name for the status. While this name is not visible anywhere, it allows other scripts to set and override that specific status. This allows you to break out of the "each script sets it own status" design.

For example, if you set a status with the name `global-status` in an `install.sh` script, you can later change that status in another script by specifying its name.