import utils

def __main__():
    lines = utils.read_input()

    result = 0

    x = 0
    for line in lines[1:]:
        x = (x + 3) % len(line)
        if line[x] == '#':
            result += 1

    print(result)

__main__()
