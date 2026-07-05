CREATE EXTENSION vector;

CREATE TABLE collections (
    id             uuid PRIMARY KEY DEFAULT uuidv4(),
    name           varchar(32),
    description    varchar(256)
);

CREATE TYPE extraction_type AS ENUM (
    'merge',
    'override'
);

CREATE TYPE extraction_status AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'failed',
    'cancelled'
);

CREATE TABLE extractions (
    id                 uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id      uuid REFERENCES collections(id) ON DELETE CASCADE,
    status             extraction_status NOT NULL DEFAULT 'pending',
    source             varchar(128) NOT NULL,
    source_type        varchar(32) NOT NULL,
    extraction_type    extraction_type NOT NULL DEFAULT 'merge',
    parameters         jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at         timestamptz NOT NULL DEFAULT timezone('utc', now())
);

CREATE TABLE documents (
    id                uuid PRIMARY KEY DEFAULT uuidv4(),
    collection_id     uuid REFERENCES collections(id) ON DELETE CASCADE,
    canonical_path    varchar(256) NOT NULL,
    checksum          bytea NOT NULL
);

CREATE TYPE indexing_status AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'failed'
);

CREATE TABLE indexing (
    id             uuid PRIMARY KEY DEFAULT uuidv4(),
    document_id    uuid REFERENCES documents(id) ON DELETE CASCADE,
    status         indexing_status NOT NULL DEFAULT 'pending',
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

CREATE FUNCTION rrf_score(rank bigint, rrf_k int DEFAULT 50)
RETURNS numeric
LANGUAGE SQL
IMMUTABLE PARALLEL SAFE
AS $$
    SELECT coalesce(1.0 / ($1 + $2), 0.0);
$$;
