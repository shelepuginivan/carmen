from pydantic import BaseModel


class ChunkEnqueued(BaseModel):
    document_id: str
    text: str


class ChunkReady(BaseModel):
    document_id: str
    text: str
    embedding: list[float]
    language: str
