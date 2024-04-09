class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        numdict = {}
        for i, n in enumerate(nums):
            if (target - n) in numdict:
                return [i, numdict[target - n]]
            numdict[n] = i
        return [-1, -1]