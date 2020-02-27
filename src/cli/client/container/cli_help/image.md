# Lucky Container Image

Create containers and set their image.

${help_message}

## Usage

`lucky container image` allows you to set and get the Docker image for a container. The way you create new containers in Lucky is to set the container image with `lucky container image set`. After setting the image of the container, you can specify other settings such as environment variables and the container will be created when the script exits.

> **Note:** The container tag or digest is **required** when setting the contianer image. Unlike Docker, Lucky will not assume that you mean to use the `latest` tag when you leave the tag unspecified.

## Examples

**Create a new Nginx container and bind 80 on the host:**

```bash
$ lucky container image set nginx:latest
$ lucky container port add 80:80
```