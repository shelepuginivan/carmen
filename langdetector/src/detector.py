from fastapi import Depends
from lingua import LanguageDetector, LanguageDetectorBuilder

from config import Config, get_config


def get_language_detector(config: Config = Depends(get_config)) -> LanguageDetector:
    builder = LanguageDetectorBuilder.from_languages(*config.get_languages())

    if config.preload:
        builder = builder.with_preloaded_language_models()

    if config.low_accuracy:
        builder = builder.with_low_accuracy_mode()

    return builder.build()
