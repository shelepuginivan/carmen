from pydantic import BaseModel


class ChunkEnqueued(BaseModel):
    document_id: int
    chunk_text: str
