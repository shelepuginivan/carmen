# Carmen

Carmen is a documentation search platform that supports similarity, full-text,
semantic, and hybrid search.

It is currently in the early stages of development. Pretty much everything is
subject to change.


## Setup

### Docker Compose

Use the provided [`compose.yml`](./compose.yml) file or create your own setup.

1.  Copy `env/.env.*.example` files to the respective `env/.env.*` files
    (e.g. `env/.env.postgres.example` to `env/.env.postgres`) and adjust the
    environment variables.

2.  Start the containers:
    ```sh
    docker compose up -d
    ```

OpenAPI web interface is available at http://localhost:5124/swagger/index.html.
RustFS web console can be accessed at http://localhost:9001/.

See documentation of individual services for details on how to configure them:
- [embedding](./embedding/README.md)
- [extractor](./extractor/README.md)
- [langdetector](./langdetector/README.md)
- [search](./search/README.md)


## License

Carmen is licensed under [MIT license](./LICENSE).
