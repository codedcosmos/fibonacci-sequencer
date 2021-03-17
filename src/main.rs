use crate::biguint::{BigInt, TextBuffer};

mod biguint;

fn main() {
    let mut a = BigInt::new(); // 0
    let mut b = BigInt::one(); // 1
    let mut c = BigInt::new(); // 0 (but doesn't matter)

    let mut text_buffer = TextBuffer::new();

    let mut i: u128 = 2;

    println!("Fibonacci Number {} = {}", 0, 0);
    println!("Fibonacci Number {} = {}", 1, 1);

    for _o in 0..10000 {
        // Calculate
        c.set_as_sum(&mut a, &mut b);
        a.set_to(&b);
        b.set_to(&c);

        // Display
        text_buffer.get_decimal(&c);

        let mut text = String::new();
        for item in &mut text_buffer {
            text.push_str(item.as_str());
        }

        println!("Fibonacci Number {} = {}", i, text);

        i+=1;
    }
}