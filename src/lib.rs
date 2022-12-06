use std::cmp::{Ordering, PartialOrd};
use std::fmt::Debug;

mod test_data {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
    pub enum Suit {
        #[default]
        Diamond = 0,
        Club,
        Heart,
        Spade,
    }

    #[derive(Debug, PartialEq, Eq, Default, Clone)]
    pub struct Poker {
        // not participate in sorting, just to highlight stability.
        pub id: i16,
        // the range of number should be [3,10).
        pub number: i16,
        // if number is equal to the other, then sort by suit.
        pub suit: Suit,
    }

    impl PartialOrd for Poker {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Poker {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.number.cmp(&other.number) {
                Ordering::Equal => self.suit.cmp(&other.suit),
                _ => self.number.cmp(&other.number),
            }
        }
    }
}
/// Sorts the slice.
///
/// This sort is stable (i.e., does not reorder equal elements) and *O* (*n*^*2*) worst-case.
///
/// # Current implementation
///
/// the algorithm is implemented for the elements with the trait [`PartialOrd`],
/// so the function may get the elements without the implementation of [`Ord`].
/// Because the order of `NAN` is not determined, the function move these elements
/// to the end of slice predefined, and will return the number of sorted elements.
///
/// # Examples
///
/// ```
/// let mut v = [-5.0, f32::NAN, 4.0, 1.0, -3.0, 2.0];
///
/// let len = bubble_sort(&mut v);
/// assert_eq!(len, v.len() - 1);
/// assert!(v == [-5.0, -3.0, 1.0, 2.0, 4.0, f32::NAN]);
/// ```
pub fn bubble_sort<T>(array: &mut [T]) -> usize
where
    T: PartialOrd + Default,
{
    if array.is_empty() {
        return 0;
    }

    // nan_index records the next position of putting the nan
    let mut nan_index = array.len() - 1;
    let mut i_range = 0..=nan_index;

    while let Some(i) = i_range.next() {
        let mut moved = false;
        let mut j_range = 0..(nan_index - i);
        while let Some(j) = j_range.next() {
            match &array[j].partial_cmp(&array[j + 1]) {
                Some(Ordering::Greater) => {
                    array.swap(j, j + 1);
                    moved = true;
                }
                None => {
                    // we should find the element that can't compare.
                    if array[j].partial_cmp(&Default::default()).is_none() {
                        array.swap(j, nan_index);
                    } else {
                        array.swap(j + 1, nan_index);
                    }
                    moved = true;
                    nan_index -= 1;
                    // in the loop of "for _ in range", rust can't change the value of range,
                    // so we need to adjust the range clearly by this way.
                    j_range = j..(nan_index - i);
                }
                _ => (),
            }
        }

        // the slice is sorted
        if !moved {
            break;
        }

        // update to return prematurely
        i_range = i + 1..=nan_index;
    }

    match array[0].partial_cmp(&Default::default()) {
        None => 0,
        _ => nan_index + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::test_data::*;
    use super::*;

    #[test]
    fn sort_empty_slice() {
        let mut array = [0; 0];
        bubble_sort(&mut array);
        assert_eq!(array, []);
    }

    #[test]
    fn sort_single_element() {
        let mut array = [1];
        bubble_sort(&mut array);
        assert_eq!(array, [1]);
    }

    #[test]
    fn sort_reverse_order_multiple_elements() {
        let mut array = [4, 3, 2, 1, 0, -1, -2];
        bubble_sort(&mut array);
        assert_eq!(array, [-2, -1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn sort_positive_order_multiple_elements() {
        let mut array = [-2, -1, 0, 1, 2, 3, 4];
        bubble_sort(&mut array);
        assert_eq!(array, [-2, -1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn sort_disorder_multiple_elements() {
        let mut array = [1, 3, 0, -1, -2, 2, 4];
        bubble_sort(&mut array);
        assert_eq!(array, [-2, -1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn sort_disorder_duplicate_elements() {
        let mut array = [1, 3, 0, -1, -2, 2, 0, -2, 3];
        bubble_sort(&mut array);
        assert_eq!(array, [-2, -2, -1, 0, 0, 1, 2, 3, 3]);
    }

    // test(_NAN) will fail because Because it cann't be determined that NAN == NAN,
    // they are tested just for ensuring bubble_sort move elements which are similiar to NAN to the end of slice.
    // However, the return value of bubble_sort must be correct.
    #[test]
    fn sort_single_NAN() {
        let mut array = [f32::NAN];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 1);
        assert_eq!(array, [f32::NAN]);
    }

    #[test]
    fn sort_two_NAN() {
        let mut array = [f32::NAN, f32::NAN];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 2);
        assert_eq!(array, [f32::NAN, f32::NAN]);
    }

    #[test]
    fn sort_three_NAN() {
        let mut array = [f32::NAN, f32::NAN, f32::NAN];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 3);
        assert_eq!(array, [f32::NAN, f32::NAN, f32::NAN]);
    }

    #[test]
    fn sort_reverse_order_multiple_elements_with_NAN() {
        let mut array = [4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, f32::NAN];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 1);
        assert_eq!(array, [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, f32::NAN]);
    }

    #[test]
    fn sort_positive_order_multiple_elements_with_NAN() {
        let mut array = [f32::NAN, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 1);
        assert_eq!(array, [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, f32::NAN]);
    }

    #[test]
    fn sort_disorder_multiple_elements_with_NAN() {
        let mut array = [1.0, 3.0, 0.0, f32::NAN, -1.0, -2.0, 2.0, 4.0];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 1);
        assert_eq!(array, [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, f32::NAN]);
    }

    #[test]
    fn sort_disorder_duplicate_elements_with_NAN() {
        let mut array = [
            1.0,
            f32::NAN,
            3.0,
            0.0,
            -1.0,
            -2.0,
            2.0,
            f32::NAN,
            0.0,
            -2.0,
            -3.0,
        ];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 2);
        assert_eq!(
            array,
            [
                -3.0,
                -2.0,
                -2.0,
                -1.0,
                0.0,
                0.0,
                1.0,
                2.0,
                3.0,
                f32::NAN,
                f32::NAN
            ]
        );
    }

    #[test]
    fn sort_elements_with_many_NAN() {
        let mut array = [f32::NAN, f32::NAN, 0.0, f32::NAN, -1.0, f32::NAN];
        let len = bubble_sort(&mut array);
        assert_eq!(len, array.len() - 4);
        assert_eq!(array, [-1.0, 0.0, f32::NAN, f32::NAN, f32::NAN, f32::NAN]);
    }

    fn generate_pokers(nums: usize, data: &[(i16, i16, Suit)]) -> Vec<Poker> {
        let mut pokers: Vec<Poker> = vec![Default::default(); nums];

        for i in 0..nums {
            pokers[i] = Poker {
                id: data[i].0,
                number: data[i].1,
                suit: data[i].2.clone(),
            }
        }
        pokers
    }

    // test struct with the trait PartialOrd
    #[test]
    fn sort_disorder_flush_elements() {
        let poker = [
            (0, 3, Suit::Spade),
            (0, 3, Suit::Club),
            (0, 3, Suit::Diamond),
            (0, 3, Suit::Heart),
        ];
        let mut pokers = generate_pokers(poker.len(), &poker);
        bubble_sort(&mut pokers);

        let poker = [
            (0, 3, Suit::Diamond),
            (0, 3, Suit::Club),
            (0, 3, Suit::Heart),
            (0, 3, Suit::Spade),
        ];
        let ordered_pokers = generate_pokers(poker.len(), &poker);
        assert_eq!(pokers, ordered_pokers);
    }

    #[test]
    fn sort_disorder_poker_elements_by_number() {
        let poker = [
            (0, 3, Suit::Spade),
            (0, 9, Suit::Club),
            (0, 8, Suit::Diamond),
            (0, 5, Suit::Heart),
        ];
        let mut pokers = generate_pokers(poker.len(), &poker);
        bubble_sort(&mut pokers);

        let poker = [
            (0, 3, Suit::Spade),
            (0, 5, Suit::Heart),
            (0, 8, Suit::Diamond),
            (0, 9, Suit::Club),
        ];
        let ordered_pokers = generate_pokers(poker.len(), &poker);
        assert_eq!(pokers, ordered_pokers);
    }

    #[test]
    fn sort_disorder_duplicate_poker_elements() {
        let poker = [
            (0, 3, Suit::Spade),
            (0, 3, Suit::Club),
            (1, 4, Suit::Diamond),
            (0, 4, Suit::Heart),
            (0, 4, Suit::Diamond),
        ];
        let mut pokers = generate_pokers(poker.len(), &poker);
        bubble_sort(&mut pokers);

        let poker = [
            (0, 3, Suit::Club),
            (0, 3, Suit::Spade),
            (1, 4, Suit::Diamond),
            (0, 4, Suit::Diamond),
            (0, 4, Suit::Heart),
        ];
        let ordered_pokers = generate_pokers(poker.len(), &poker);
        assert_eq!(pokers, ordered_pokers);
    }
}
