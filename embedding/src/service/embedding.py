from dataclasses import dataclass

import httpx
from sentence_transformers import SentenceTransformer

from models.config import Config


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
        res = httpx.post(self.__langdector)
        if res.status_code != 200:
            raise RuntimeError("cannot detect language")

        lang = res.text
        embedding = self.__models[lang].encode(text).tolist()
        return EmbeddingResult(lang, embedding)
