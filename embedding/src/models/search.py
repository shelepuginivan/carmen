from pydantic import BaseModel


class SearchRequest(BaseModel):
    query: str
    response_topic: str


class SearchResponse(BaseModel):
    embedding: list[float]
