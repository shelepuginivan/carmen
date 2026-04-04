import httpx
from common.config import Config
from pydantic.dataclasses import dataclass
from sentence_transformers import SentenceTransformer


@dataclass
class EmbeddingResult:
    language: str
    embedding: list[float]


class EmbeddingService:
    def __init__(self, config: Config) -> None:
        self.__models: dict[str, SentenceTransformer] = {}
        self.__langdector = str(config.langdetector_url)

        for lang, model in config.model.items():
            self.__models[lang] = SentenceTransformer(
                model,
                cache_folder=config.sentence_transformers_home,
                local_files_only=config.sentence_transformers_home is not None,
            )

    def generate_embedding(self, text: str) -> EmbeddingResult:
        res = httpx.post(self.__langdector, content=text)
        if res.status_code != 200:
            raise RuntimeError("cannot detect language")

        lang = res.text
        embedding = self.__models[lang].encode(text).tolist()
        return EmbeddingResult(lang, embedding)

    async def generate_embedding_async(self, text: str) -> EmbeddingResult:
        async with httpx.AsyncClient() as client:
            res = await client.post(self.__langdector, content=text)
            if res.status_code != 200:
                raise RuntimeError("cannot detect language")

            lang = res.text
            embedding = self.__models[lang].encode(text).tolist()
            return EmbeddingResult(lang, embedding)
