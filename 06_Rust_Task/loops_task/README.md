# Subarray XOR Problem (Rust)

##  Problem Statement

Given an array of integers, print the **first contiguous subarray**
whose **bitwise XOR of only positive numbers is zero**.

### Rules
- Consider **only positive numbers** for XOR calculation
- The subarray must be **contiguous**
- Print the **first** such subarray found
- If no such subarray exists, print `not possible...`

---

##   Example

**Input**

[-1, 2, 3, 3, 2];


**Output**
2 3 3 2

---

##  Approach

1. Use a `left` pointer to mark the start of a subarray
2. Expand a `right` pointer to explore all possible subarrays
3. Compute XOR of **only positive numbers**
4. Stop immediately when XOR becomes `0`
5. Use labeled `break` to exit nested loops efficiently



