def main() -> None:
    config = Config()  # type: ignore

    match config.mode:
        case "chunks":
            from chunks.main import main

            main()
        case "search":
            from search.main import main

            main()


if __name__ == "__main__":
    main()
