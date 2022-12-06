use std::cmp::{Ordering, PartialOrd};

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
        // not participate in sorting, just to highlight stability
        pub id: i16,
        // the range of number is [3,10)
        pub number: i16,
        pub suit: Suit
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
                _ => self.number.cmp(&other.number)
            }
        }
    }


}
/// bubble_sort 
pub fn bubble_sort<T>(array: &mut [T])
    where T: PartialOrd,
{
    // nan_index records the next position of putting the nan
    let mut nan_index = array.len() - 1;
    for i in 0..array.len() {
        for j in (i+1)..array.len() {
            match &array[i].partial_cmp(&array[j]) { 
                Some(Ordering::Greater) => array.swap(i, j),
                None => {
                    if array[i] == f32::NAN || array[i] == f64::NAN {
                        array.swap(i, nan_index);
                    } else {
                        array.swap(j, nan_index);
                    }
                    nan_index -= 1;
                },
                _ => ()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_data::*;

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

    #[test]
    fn sort_disorder_flush_elements() {
        let poker = [(0, 3, Suit::Spade), (0, 3, Suit::Club), (0, 3, Suit::Diamond), (0, 3, Suit::Heart)];
        let mut pokers = generate_pokers(poker.len(), &poker);
        bubble_sort(&mut pokers);

        let poker = [(0, 3, Suit::Diamond), (0, 3, Suit::Club), (0, 3, Suit::Heart), (0, 3, Suit::Spade)];
        let ordered_pokers = generate_pokers(poker.len(), &poker);
        assert_eq!(pokers, ordered_pokers);
    }

    #[test]
    fn sort_disorder_poker_elements_by_number() {
        let poker = [(0, 3, Suit::Spade), (0, 9, Suit::Club), (0, 8, Suit::Diamond), (0, 5, Suit::Heart)];
        let mut pokers = generate_pokers(poker.len(), &poker);
        bubble_sort(&mut pokers);

        let poker = [(0, 3, Suit::Spade), (0, 5, Suit::Heart), (0, 8, Suit::Diamond), (0, 9, Suit::Club)];
        let ordered_pokers = generate_pokers(poker.len(), &poker);
        assert_eq!(pokers, ordered_pokers);
    }

    #[test]
    fn sort_disorder_duplicate_poker_elements() {
        let poker = [(0, 3, Suit::Spade), (0, 3, Suit::Club), (1, 4, Suit::Diamond), (0, 4, Suit::Heart), (0, 4, Suit::Diamond)];
        let mut pokers = generate_pokers(poker.len(), &poker);
        bubble_sort(&mut pokers);

        let poker = [(0, 3, Suit::Club), (0, 3, Suit::Spade), (1, 4, Suit::Diamond), (0, 4, Suit::Diamond), (0, 4, Suit::Heart)];
        let ordered_pokers = generate_pokers(poker.len(), &poker);
        assert_eq!(pokers, ordered_pokers);
    }
    
}
