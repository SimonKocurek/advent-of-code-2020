def read_input() -> list[str]:
    with open("input", "r") as f:
        return [line.strip() for line in f.readlines()]
