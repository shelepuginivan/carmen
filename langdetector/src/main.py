from fastapi import FastAPI, Body
from fastapi.responses import PlainTextResponse


app = FastAPI()


@app.post("/detect", response_class=PlainTextResponse)
def read_item(body: str = Body(media_type="text/plain")):
    return body
