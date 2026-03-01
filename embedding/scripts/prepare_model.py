import os

from sentence_transformers import SentenceTransformer


model_name = os.getenv("SENTENCE_TRANSFORMERS_MODEL", "")

if model_name != "":
    # NOTE: sentence_transformers uses SENTENCE_TRANSFORMERS_HOME environment
    #       variable for cache, which is defined in Containerfile.
    SentenceTransformer(model_name)
