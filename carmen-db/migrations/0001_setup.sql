CREATE TYPE collection_extraction_status AS ENUM (
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
    source         varchar(128)
);

CREATE TABLE collection_extractions (
    id               uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id    uuid REFERENCES collections(id),
    status           collection_extraction_status NOT NULL DEFAULT 'pending',
    created_at       timestamptz NOT NULL DEFAULT timezone('utc', now())
);
