use itertools::Itertools;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub min: u64,
    pub max: u64,
}

impl Interval {
    pub fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    pub fn merge(self, other: Interval) -> Option<Interval> {
        #[allow(unused_assignments)]
        let mut merged = self.merge_left(other);
        if merged.is_none() {
            merged = other.merge_left(self);
        }

        merged
    }

    fn merge_left(self, other: Interval) -> Option<Interval> {
        let mut merged = None;
        if self.min <= other.min && other.min <= self.max && other.max >= self.max {
            // [-----]
            //     [-----]
            merged = Some(Interval::new(self.min, other.max));
        } else if self.min <= other.min && other.max <= self.max {
            merged = Some(self)
        }

        merged
    }

    pub fn reduce(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut merged_indices = HashSet::<usize>::new();
        let mut reduced = Vec::new();
        for i in 0..intervals.len() {
            if merged_indices.contains(&i) {
                continue;
            }
            let mut current = intervals[i];
            #[allow(clippy::needless_range_loop)]
            for j in i + 1..intervals.len() {
                if merged_indices.contains(&j) {
                    continue;
                }
                if let Some(merged) = current.merge(intervals[j]) {
                    current = merged;
                    merged_indices.insert(i);
                    merged_indices.insert(j);
                }
            }
            reduced.push(current);
        }

        reduced
    }

    pub fn intersect(self, other: Interval) -> Option<Interval> {
        if self == other {
            return Some(self);
        }

        if self.min <= other.min && self.max > other.min && self.max < other.max {
            // [-----]
            //     [-----]
            // or
            // [-----]
            // [--------]
            return Some(Interval::new(other.min, self.max));
        } else if self.min > other.min && self.min < other.max && self.max >= other.max {
            //         [-----]
            //     [-----]
            // or
            //         [-----]
            //     [---------]
            return Some(Interval::new(self.min, other.max));
        } else if other.min <= self.min && other.max >= self.max {
            //     [-----]
            //   [-------------]
            return Some(Interval::new(self.min, self.max));
        } else if self.min <= other.min && self.max >= other.max {
            //   [-------------]
            //     [-----]
            return Some(Interval::new(other.min, other.max));
        }

        None
    }

    pub fn complement(self, other: Interval) -> Vec<Interval> {
        let mut res = Vec::with_capacity(2);
        if self == other {
            return res;
        }

        if self.min <= other.min && self.max > other.min && self.max < other.max {
            // [-----]
            //     [-----]
            // or
            // [-----]
            // [--------]
            res.push(Interval::new(self.max + 1, other.max));
        } else if self.min > other.min && self.min < other.max && self.max >= other.max {
            //         [-----]
            //     [-----]
            // or
            //         [-----]
            //     [---------]
            res.push(Interval::new(other.min, self.min - 1));
        } else if other.min < self.min && other.max > self.max {
            //     [-----]
            //   [-------------]
            res.push(Interval::new(other.min, self.min - 1));
            res.push(Interval::new(self.max + 1, other.max));
        }

        res
    }

    /// Compute the complements to cover the same range as the current interval if we do an union between the input and the response.
    pub fn complement_for_subs(self, others: Vec<Interval>) -> Vec<Interval> {
        if others.is_empty() {
            return vec![self];
        }
        let mut left_cursor = self.min;
        let mut res = Vec::new();
        for i in others.into_iter().sorted_by_key(|i| i.min) {
            if left_cursor < i.min {
                res.push(Interval::new(left_cursor, i.min - 1));
            }
            left_cursor = i.max + 1;
        }
        if left_cursor < self.max {
            res.push(Interval::new(left_cursor, self.max));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_should_compute_complement_for_subs_with_empty_list() {
        let result = Interval::new(0, 10).complement_for_subs(vec![]);
        assert_eq!(result, vec![Interval::new(0, 10)]);
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_with_empty_list_and_smallest_interval() {
        let result = Interval::new(10, 10).complement_for_subs(vec![]);
        assert_eq!(result, vec![Interval::new(10, 10)]);
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_if_list_contains_one_subset() {
        let result = Interval::new(0, 10).complement_for_subs(vec![Interval::new(3, 5)]);
        assert_eq!(result, vec![Interval::new(0, 2), Interval::new(6, 10)]);
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_if_list_contains_one_subset_having_same_start(
    ) {
        let result = Interval::new(0, 10).complement_for_subs(vec![Interval::new(0, 5)]);
        assert_eq!(result, vec![Interval::new(6, 10)]);
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_if_list_contains_one_subset_having_same_end(
    ) {
        let result = Interval::new(0, 10).complement_for_subs(vec![Interval::new(3, 10)]);
        assert_eq!(result, vec![Interval::new(0, 2)]);
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_with_complex_list() {
        let result = Interval::new(0, 100).complement_for_subs(vec![
            Interval::new(3, 10),
            Interval::new(20, 30),
            Interval::new(40, 50),
            Interval::new(60, 70),
            Interval::new(80, 90),
        ]);
        assert_eq!(
            result,
            vec![
                Interval::new(0, 2),
                Interval::new(11, 19),
                Interval::new(31, 39),
                Interval::new(51, 59),
                Interval::new(71, 79),
                Interval::new(91, 100)
            ]
        );
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_with_complex_list_containing_bounds() {
        let result = Interval::new(0, 100).complement_for_subs(vec![
            Interval::new(0, 10),
            Interval::new(20, 30),
            Interval::new(40, 50),
            Interval::new(60, 70),
            Interval::new(80, 100),
        ]);
        assert_eq!(
            result,
            vec![
                Interval::new(11, 19),
                Interval::new(31, 39),
                Interval::new(51, 59),
                Interval::new(71, 79),
            ]
        );
    }

    #[test]
    fn test_interval_should_compute_complement_for_subs_with_complex_list_covering_entire_interval()
    {
        let result = Interval::new(0, 100).complement_for_subs(vec![
            Interval::new(0, 10),
            Interval::new(11, 30),
            Interval::new(31, 50),
            Interval::new(51, 70),
            Interval::new(71, 100),
        ]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_interval_should_compute_complement_if_second_overlap_end_of_first() {
        let result = Interval::new(0, 10).complement(Interval::new(8, 15));
        assert_eq!(result, vec![Interval::new(11, 15)]);
    }

    #[test]
    fn test_interval_should_compute_complement_if_first_overlap_end_of_second() {
        let result = Interval::new(8, 15).complement(Interval::new(0, 10));
        assert_eq!(result, vec![Interval::new(0, 7)]);
    }

    #[test]
    fn test_interval_should_bot_create_any_complement_if_first_fully_overlap_second() {
        let result = Interval::new(0, 15).complement(Interval::new(4, 10));
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_interval_should_create_two_complements_if_second_fully_overlap_first() {
        let result = Interval::new(3, 15).complement(Interval::new(0, 20));
        assert_eq!(result, vec![Interval::new(0, 2), Interval::new(16, 20)]);
    }

    #[test]
    fn test_interval_should_create_one_complement_if_second_fully_overlap_first_at_beginning() {
        let result = Interval::new(0, 15).complement(Interval::new(0, 100));
        assert_eq!(result, vec![Interval::new(16, 100)]);
    }

    #[test]
    fn test_interval_should_create_one_complement_if_second_fully_overlap_first_at_end() {
        let result = Interval::new(80, 100).complement(Interval::new(0, 100));
        assert_eq!(result, vec![Interval::new(0, 79)]);
    }

    #[test]
    fn test_interval_should_be_merged_if_self_contain_other() {
        let result = Interval::new(0, 10).merge(Interval::new(2, 5));
        assert_eq!(result, Some(Interval::new(0, 10)));
    }

    #[test]
    fn test_interval_should_be_merged_if_other_contain_self() {
        let result = Interval::new(3, 10).merge(Interval::new(0, 15));
        assert_eq!(result, Some(Interval::new(0, 15)));
    }

    #[test]
    fn test_interval_should_be_merged_if_equals() {
        let result = Interval::new(0, 10).merge(Interval::new(0, 10));
        assert_eq!(result, Some(Interval::new(0, 10)));
    }

    #[test]
    fn test_interval_should_be_merged_if_second_overlap_end_of_first() {
        let result = Interval::new(0, 10).merge(Interval::new(3, 20));
        assert_eq!(result, Some(Interval::new(0, 20)));
    }

    #[test]
    fn test_interval_should_be_merged_if_first_overlap_end_of_second() {
        let result = Interval::new(3, 10).merge(Interval::new(0, 5));
        assert_eq!(result, Some(Interval::new(0, 10)));
    }

    #[test]
    fn test_interval_should_reduce_list_of_intervals() {
        let result = Interval::reduce(vec![
            Interval::new(3, 10),
            Interval::new(0, 25),
            Interval::new(28, 30),
            Interval::new(29, 50),
            Interval::new(100, 200),
            Interval::new(4, 12),
        ]);
        assert_eq!(
            result,
            vec![
                Interval::new(0, 25),
                Interval::new(28, 50),
                Interval::new(100, 200)
            ]
        );
    }

    #[test]
    fn test_interval_should_intersect_overlapping_intervals() {
        let result = Interval::new(3, 10).intersect(Interval::new(0, 5));
        assert_eq!(result, Some(Interval::new(3, 5)));
    }

    #[test]
    fn test_interval_should_intersect_with_first_containing_second() {
        let result = Interval::new(3, 10).intersect(Interval::new(4, 5));
        assert_eq!(result, Some(Interval::new(4, 5)));
    }

    #[test]
    fn test_interval_should_intersect_with_first_containing_second_or_equal() {
        let result = Interval::new(4, 10).intersect(Interval::new(4, 5));
        assert_eq!(result, Some(Interval::new(4, 5)));
    }

    #[test]
    fn test_interval_should_intersect_with_second_containing_first() {
        let result = Interval::new(3, 10).intersect(Interval::new(0, 25));
        assert_eq!(result, Some(Interval::new(3, 10)));
    }
}
