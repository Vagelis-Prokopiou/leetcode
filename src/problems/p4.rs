/*
4. Median of Two Sorted Arrays

Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
Follow up: The overall run time complexity should be O(log (m+n)).
*/

// #![feature(test)]
// extern crate test;

pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        //let mut sorted_array: Vec<i32> = Vec::with_capacity(nums1.len() + nums2.len());
        let mut sorted_array: Vec<i32> = Vec::new();
        for num in nums1 {
            sorted_array.push(num);
        }
        for num in nums2 {
            sorted_array.push(num);
        }
        sorted_array.sort();

        if sorted_array.len() == 1 {
            return sorted_array[0] as f64;
        }
        if sorted_array.len() == 2 {
            let a = sorted_array[0] as f64;
            let b = sorted_array[1] as f64;
            let result = (a + b) / 2.0;
            return result;
        }

        if sorted_array.len() % 2 == 0 {
            let middle = sorted_array.len() / 2;
            let a = sorted_array[middle - 1] as f64;
            let b = sorted_array[middle] as f64;
            let result = (a + b) / 2.0;
            return result;
        }

        return sorted_array[sorted_array.len() / 2] as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test_find_median_sorted_arrays() {
        let nums1 = [1, 3];
        let nums2 = [2];
        assert_eq!(
            Solution::find_median_sorted_arrays(Vec::from(nums1), Vec::from(nums2)),
            2.0
        );

        let nums1 = [1, 2];
        let nums2 = [3, 4];
        assert_eq!(
            Solution::find_median_sorted_arrays(Vec::from(nums1), Vec::from(nums2)),
            2.5
        );

        let nums1 = [];
        let nums2 = [2, 3];
        assert_eq!(
            Solution::find_median_sorted_arrays(Vec::from(nums1), Vec::from(nums2)),
            2.5
        );
    }

    // #[bench]
    // fn bench_find_median_sorted_arrays(b: &mut Bencher) {
    //     let nums1 = [1, 2];
    //     let nums2 = [3, 4];
    //     b.iter(|| Solution::find_median_sorted_arrays(Vec::from(nums1), Vec::from(nums2)));
    // }
}