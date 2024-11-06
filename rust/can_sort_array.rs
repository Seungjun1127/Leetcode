impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        // if they have same number of 1's in bit, they are same family.
        let family = |n: i32| {
            n.count_ones()
        };

        let mut curr_max = 0;
        let mut prev_max = 0;
        let mut curr_family = family(nums[0]);

        for n in nums {
            // current family member needs to taller than prev family's tallest one.
            if prev_max > n {
                return false;
            }

            if family(n) != curr_family {
                // now n has different family with >>current family<<
                prev_max = curr_max;

                // n should be taller than right before family.
                if prev_max > n {
                    return false;
                }

                // if it's okay -> n can make new family.
                curr_family = family(n);
                curr_max = n;

            } else {
                // if n has same family with before
                curr_max = curr_max.max(n);
            }
        }
        true
    }
}
