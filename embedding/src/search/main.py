import uvicorn

from .app import app


def main() -> None:
    uvicorn.run(app)
