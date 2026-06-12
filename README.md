# Carmen

Carmen is a documentation search platform that supports similarity, full-text,
semantic, and hybrid search.

It is currently in the early stages of development. Pretty much everything is
subject to change.

You are probably looking for the PoC implementation, which is tagged as
[`0.1.0-poc`](https://github.com/shelepuginivan/carmen/tree/v0.1.0-poc).


## Setup

### Building images

```sh
podman build --target carmen-indexer -t localhost/carmen-indexer:latest .
podman build --target carmen-migrations -t localhost/carmen-migrations:latest .
podman build --target carmen-search -t localhost/carmen-search:latest .
```

## License

Carmen is licensed under [MIT license](./LICENSE).
