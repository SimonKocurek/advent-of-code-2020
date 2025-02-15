import utils
import re

def __main__():
    lines = utils.read_input()

    result = 0

    for line in lines:
        range_min, range_max, char, _, sequence = re.split(r'[\- :]', line)
        if int(range_min) <= sequence.count(char) <= int(range_max):
            result += 1

    print(result)

__main__()
