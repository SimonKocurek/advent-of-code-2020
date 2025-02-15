import utils

def __main__():
    lines = utils.read_input()

    result = 1
    for xDiff, yDiff in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]:
        result *= trees_on_slope(lines, xDiff=xDiff, yDiff=yDiff)
    print(result)

def trees_on_slope(lines: list[str], xDiff: int, yDiff: int) -> int:
    result = 0
    x = 0
    
    for y in range(0, len(lines), yDiff):
        line = lines[y]

        if line[x] == '#':
            result += 1

        x = (x + xDiff) % len(line)

    return result

__main__()
