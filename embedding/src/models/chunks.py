from pydantic import BaseModel


class ChunkEnqueued(BaseModel):
    document_id: int
    text: str


class ChunkReady(BaseModel):
    document_id: int
    text: str
    embedding: list[float]
