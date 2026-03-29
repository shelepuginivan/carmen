from fastapi import Depends, FastAPI, Body, HTTPException
from fastapi.responses import PlainTextResponse
from lingua import LanguageDetector

from config import Config, get_config
from detector import get_language_detector


app = FastAPI(
    docs_url=None,
    redoc_url=None,
)


@app.post("/detect", response_class=PlainTextResponse)
async def detect_language(
    text: str = Body(media_type="text/plain"),
    config: Config = Depends(get_config),
    detector: LanguageDetector = Depends(get_language_detector),
):
    lang = detector.detect_language_of(text) or config.get_fallback_language()
    if lang is None:
        raise HTTPException(status_code=404, detail="cannot detect language reliably")
    return lang.name.lower()
