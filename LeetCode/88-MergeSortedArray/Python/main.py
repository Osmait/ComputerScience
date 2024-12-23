from typing import List


# V1
# class Solution:
#     def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
#         for i, v in enumerate(nums2):
#             pivot = i + m
#             nums1[pivot] = v
#         nums1.sort()
#


# V2
class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        while m > 0 and n > 0:
            if nums1[m - 1] >= nums2[n - 1]:
                nums1[m + n - 1] = nums1[m - 1]
                m -= 1
            else:
                nums1[m + n - 1] = nums2[n - 1]
                n -= 1
            print(nums1)
        if n > 0:
            nums1[:n] = nums2[:n]


if __name__ == "__main__":
    nums1 = [1, 2, 3, 0, 0, 0]
    m = 3
    nums2 = [2, 5, 6]
    n = 3
    solution = Solution()
    solution.merge(nums1, m, nums2, n)
    print(nums1)
