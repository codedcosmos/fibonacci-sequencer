use primitive_types::U256;

fn main() {
    let mut a = U256::from(0);
    let mut b = U256::from(1);
    let mut c = U256::from(0);

    let mut i = U256::from(0);

    loop {
        c = a+b;
        a = b;
        b = c;

        i+=U256::from(1);

        println!("Fibonacci Number {} = {}", i, c);
    }
}
