mod bitmap;

fn main() {
    println!("Hello, world!");

    let mut bitmap = bitmap::Bitmap::new();

    println!("{}", bitmap);

    bitmap.set_bits(1, 3);

    println!("{}", bitmap);

    bitmap.set_bit(5);

    println!("{}", bitmap);

    bitmap.clear_bit(2);

    println!("{}", bitmap);
}