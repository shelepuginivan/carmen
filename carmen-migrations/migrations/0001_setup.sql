CREATE TYPE collection_task_status AS ENUM ('pending', 'extracting', 'indexing', 'completed', 'failed');

CREATE TABLE collections (
    id             uuid PRIMARY KEY DEFAULT uuidv4(),
    name           varchar(32) UNIQUE NOT NULL,
    description    varchar(256),
    source         varchar(128)
);

CREATE TABLE collection_tasks (
    id               uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id    uuid REFERENCES collections(id),
    status           collection_task_status NOT NULL DEFAULT 'pending'
);
