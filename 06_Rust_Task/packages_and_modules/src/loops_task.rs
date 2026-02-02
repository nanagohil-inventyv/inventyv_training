pub fn run() {
    // Given an array of integers, print the first subarray
    // whose bitwise XOR of only positive numbers is zero.

    let nums: [i32; 5] = [-1, 2, 3, 3, 2];

    let mut left = 0;
    let mut right = 0;

    let mut found = false;

    'outer: loop {
        if  left >= nums.len() {
            break;
        }
        right = left;

        while right < nums.len() {
            let mut xor_val = nums[left];
            let mut i = left + 1;

            while i <= right && i < nums.len() {
                if nums[i] >= 0 {
                    xor_val ^= nums[i];
                }
                i += 1;
            }

            if xor_val == 0 {
                found = true;
                break 'outer;
            }

            right += 1;
        }

        left += 1;
    }

    if !found {
        println!("not possible...");
    } else {
        for i in left..=right {
            print!("{} ", nums[i]);
        }
        println!();
    }
}

