from fastapi import Depends, FastAPI, Body

from .depends import get_service
from service.embedding import EmbeddingService


app = FastAPI(docs_url=None, redoc_url=None)


@app.post("/embedding")
async def generate_embedding(
    query: str = Body(media_type="text/plain"),
    service: EmbeddingService = Depends(get_service),
):
    return await service.generate_embedding_async(query)
