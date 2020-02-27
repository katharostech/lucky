# Lucky Container Volume

Add and remove volumes from the container.

${help_message}

## Usage

The `lucky container volume` command can be used to add persistent volumes to your containers. The source for these volumes can be either an absolute path on the host or a named volume which will be automatically place in a dir in `/var/lib/lucky/[unit_name]/volumes/[volume_name]`.

> **Warning:** Lucky does **not** behave the same as Docker when mounting a new named volume to a non-empty directory in the container. If you mount a new named Lucky volume to a non-empty path in the container, the contents of that directory, in the container, will be masked by the empty volume that is being mounted to that location. This is contrary to Docker's behavior where a new named volume will inherit the initial contents of the target dir.

## Examples

**Mount `/path/on/host` to `/data` in the container:**

    $ lucky container volume add /path/on/host /data

**Mount a volume named `attachments` to `/var/lib/app/attachments` in the container:** The data will be persisted in a directory such as `/var/lib/lucky/my_charm_3/volumes/attachments` on the host.

    $ lucky container volume add attachments /var/lib/app/attachments

**Get the source path of a volume given the target path in the container:**

    $ lucky container volume get /data
    /path/on/host

**List all of the volumes:**

    $ lucky container volume get
    /path/on/host:/data
    attachments:/var/lib/app/attachments

**Remove a volume, given the mountpoint in the container:** This will **not** delete any data, it will just unmount it from the container.

    $ lucky container volume remove /data

**Remove a volume *and* its data:** This will ***permanently delete*** any data in the volume.

    $ lucky container volume remove --delete-data /var/lib/app/attachments
