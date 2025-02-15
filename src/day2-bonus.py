import utils
import re

def __main__():
    lines = utils.read_input()

    result = 0

    for line in lines:
        first, second, char, _, sequence = re.split(r'[\- :]', line)
        if (sequence[int(first) - 1] == char) ^ (sequence[int(second) - 1] == char):
            result += 1

    print(result)

__main__()
