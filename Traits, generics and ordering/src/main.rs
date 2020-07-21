use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq)]
struct Vector2d<T>
{
    x: T,
    y: T,
}

// Partial ordering means that two elements might neither
// be less, greater nor equal to each other, as with vectors!

impl<T: Ord> PartialOrd for Vector2d<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            _ => None,
        }
    }
}
impl<T: fmt::Display> fmt::Display for Vector2d<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

fn get_partially_ordered_extreme<T: PartialOrd>(p_list: &[T], p_ordering: Ordering) -> &T {
    let mut result = &p_list[0];

    for item in &p_list[1..] {
        if item.partial_cmp(result) == Some(p_ordering) {
            result = item;
        }
    }

    result
}
fn get_partially_ordered_min<T: PartialOrd>(p_list: &[T]) -> &T {
    get_partially_ordered_extreme(p_list, Ordering::Less)
}
fn get_partially_ordered_max<T: PartialOrd>(p_list: &[T]) -> &T {
    get_partially_ordered_extreme(p_list, Ordering::Greater)
}

fn main() {
    let list = [
        Vector2d{x: 4, y: 9}, Vector2d{x: 2, y: 5}, 
        Vector2d{x: 103, y: -51}, Vector2d{x: 44, y: 42}, 
        Vector2d{x: 4, y: 9},
        Vector2d{x: 69, y: 2},
    ];

    let smallest = get_partially_ordered_min(&list);
    println!("Smallest: {}", smallest);
    let largest = get_partially_ordered_max(&list);
    println!("Largest: {}", largest);

    let first = &list[0];
    let second = &list[4];
    println!("{} == {}: {}", first, second, first == second);
    println!("{} > {}: {}", first, second, first > second);
    println!("{} < {}: {}", first, second, first < second);
}
