import utils

def __main__():
    lines = utils.read_input()
    nums = [int(line) for line in lines]
    target = 2020

    print(three_sum(nums, target))

def three_sum(nums: list[int], target: int) -> int:
    seen = {num: i for i, num in enumerate(nums)}

    for i, num in enumerate(nums):
        for j in range(i + 1, len(nums)):
            num2 = nums[j]
            num3 = target - num - num2
            
            num3Idx = seen.get(num3)
            if num3Idx is not None and num3Idx != i and num3Idx != j:
                return num * num2 * num3

__main__()
