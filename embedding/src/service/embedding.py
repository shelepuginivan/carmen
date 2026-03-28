import logging
from dataclasses import dataclass

from lingua import Language, LanguageDetectorBuilder
from sentence_transformers import SentenceTransformer
from torch import Tensor

from models.config import EmbeddingConfig


@dataclass
class EmbeddingResult:
    language: Language
    embedding: Tensor


class EmbeddingService:
    def __init__(self, cfg: EmbeddingConfig) -> None:
        self.__models: dict[Language, SentenceTransformer] = {}
        self.__fallback = cfg.get_fallback_language()

        for lang, model in cfg.model.items():
            try:
                l = Language.from_str(lang)
            except ValueError:
                logging.warning(f"skipping unknown language '{lang}'")
                continue
            self.__models[l] = SentenceTransformer(
                model,
                cache_folder=cfg.sentence_transformers_home,
                local_files_only=cfg.sentence_transformers_home is not None,
            )

        self.__detector = (
            LanguageDetectorBuilder.from_languages(*self.__models.keys())
            .with_low_accuracy_mode()
            .build()
        )

    def generate_embedding(self, text: str) -> EmbeddingResult:
        lang = self.__detector.detect_language_of(text) or self.__fallback
        if lang is None:
            raise RuntimeError("cannot detect language")
        embedding = self.__models[lang].encode(text)
        return EmbeddingResult(lang, embedding)
