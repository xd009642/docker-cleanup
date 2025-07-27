# docker-cleanup

I always wish I could do stuff like "delete all images with this name made
before this date" and other such queries with docker/podman. Well here it
is. I've made it and I used trustfall.

## What can I do?

Run queries like:

```
docker-cleanup ls --created-after 2025-07-01T00:00:00Z --larger-than 1GB --name-contains work
```

To list something like all docker images created after the start of July 2025
which are larger than 1GB and contain the string "work" in the name.

There's also regex matching, exact string matching and I can add more as I desire.

Command wise there's:

1. `ls` - list the images and their sizes
2. `rm` - remove the images
3. `size` - prints the number of images and the size of all of them

## License

This is licensed under the MIT license.
