import logging
from typing import Iterable

from lingua import IsoCode639_1, IsoCode639_3, Language
from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    languages: list[str]
    fallback_language: str | None = None
    low_accuracy: bool = False
    preload: bool = True

    def get_languages(self) -> Iterable[Language]:
        for lang in self.languages:
            try:
                yield self.__lang_from_str(lang)
            except ValueError:
                logging.warning(f"unknown language: {lang}")

    def get_fallback_language(self) -> Language | None:
        if self.fallback_language is not None:
            return self.__lang_from_str(self.fallback_language)

    @staticmethod
    def __lang_from_str(s: str) -> Language:
        try:
            return Language.from_iso_code_639_1(IsoCode639_1.from_str(s))
        except ValueError:
            pass

        try:
            return Language.from_iso_code_639_3(IsoCode639_3.from_str(s))
        except ValueError:
            pass

        return Language.from_str(s)

    model_config = SettingsConfigDict(env_prefix="CARMEN_LANGDETECTOR_")


def get_config() -> Config:
    return Config()  # type: ignore
