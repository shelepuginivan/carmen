import os


def main() -> None:
    mode = os.getenv("CARMEN_EMBEDDING_WORKING_MODE")

    match mode:
        case "chunks":
            from chunks.main import main

            main()
        case "search":
            from search.main import main

            main()
        case _:
            raise RuntimeError(f"unknown working mode: {mode}")


if __name__ == "__main__":
    main()
