import utils

def __main__():
    lines = utils.read_input()
    nums = [int(line) for line in lines]
    target = 2020

    print(two_sum(nums, target))

def two_sum(nums: list[int], target: int) -> int:
    seen = set()
    for num in nums:
        needed = target - num
        if needed in seen:
            return num * needed
        seen.add(num)

__main__()
