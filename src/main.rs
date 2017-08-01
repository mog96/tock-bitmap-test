mod bitmap;

fn main() {
    println!("Hello, world!");

    let mut bitmap = bitmap::Bitmap::new();
    println!("{}", bitmap);

    let mut no_overlap = bitmap.set_bits(1, 3);
    println!("{}", bitmap);

    assert!(no_overlap);

    bitmap.set_bit(5);
    println!("{}", bitmap);

    bitmap.clear_bit(2);
    println!("{}", bitmap);

    no_overlap = bitmap.set_bits(1, 6);
    println!("{}", bitmap);

    assert!(!no_overlap);

    no_overlap = bitmap.set_bits(6, 8);
    println!("{}", bitmap);

    assert!(no_overlap);
}