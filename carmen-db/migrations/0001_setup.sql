CREATE EXTENSION vector;

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

CREATE TYPE collection_extraction_type AS ENUM (
    'merge',
    'override'
);

CREATE TABLE collection_extractions (
    id                 uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id      uuid REFERENCES collections(id) ON DELETE CASCADE,
    status             status NOT NULL DEFAULT 'pending',
    source             varchar(128) NOT NULL,
    source_type        varchar(32) NOT NULL,
    extraction_type    collection_extraction_type NOT NULL DEFAULT 'merge',
    created_at         timestamptz NOT NULL DEFAULT timezone('utc', now())
);

CREATE TABLE documents (
    id                uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id     uuid REFERENCES collections(id) ON DELETE CASCADE,
    canonical_path    varchar(256) NOT NULL,
    checksum          bytea NOT NULL
);

CREATE TABLE document_indexing (
    id             uuid PRIMARY KEY DEFAULT uuidv4(),
    document_id    uuid REFERENCES documents(id) ON DELETE CASCADE,
    status         status NOT NULL DEFAULT 'pending',
    created_at     timestamptz NOT NULL DEFAULT timezone('utc', now())
);

CREATE TABLE chunks (
    id             uuid PRIMARY KEY DEFAULT uuidv4(),
    document_id    uuid REFERENCES documents(id) ON DELETE CASCADE,
    text           text NOT NULL,
    language       regconfig NOT NULL DEFAULT 'simple',
    fts_vector     tsvector GENERATED ALWAYS AS (to_tsvector(language, text)) STORED,
    embedding      vector NOT NULL
);
