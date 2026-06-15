CREATE TYPE status AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'failed',
    'cancelled'
);

CREATE TABLE collections (
    id             uuid PRIMARY KEY DEFAULT uuidv4(),
    name           varchar(32) UNIQUE NOT NULL,
    description    varchar(256),
    url            varchar(128),
    source         varchar(32)
);

CREATE TABLE collection_extractions (
    id               uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id    uuid REFERENCES collections(id) ON DELETE CASCADE,
    status           status NOT NULL DEFAULT 'pending',
    created_at       timestamptz NOT NULL DEFAULT timezone('utc', now())
);

CREATE TABLE documents (
    id               uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id    uuid REFERENCES collections(id),
    canonical_path   varchar(256) NOT NULL,
    checksum         bytea NOT NULL
);

CREATE TABLE document_indexing (
    id               uuid PRIMARY KEY DEFAULT uuidv4(),
    document_id      uuid REFERENCES documents(id) ON DELETE CASCADE,
    status           status NOT NULL DEFAULT 'pending',
    created_at       timestamptz NOT NULL DEFAULT timezone('utc', now())
);
