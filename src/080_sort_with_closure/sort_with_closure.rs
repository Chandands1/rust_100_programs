#[derive(Debug)] // Add Debug trait to allow printing with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Create a mutable vector of Rectangles.
    let mut list = vec![
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // Print the list before sorting.
    println!("Before sorting: {:?}", list);

    // Use sort_by with a closure to sort the rectangles by their width in ascending order.
    // The closure takes two references to elements in the slice, a and b,
    // and returns an Ordering.
    // The cmp method on u32 returns an Ordering (Less, Equal, or Greater).
    list.sort_by(|a, b| a.width.cmp(&b.width));

    // Print the list after sorting.
    println!("After sorting by width: {:?}", list);
}