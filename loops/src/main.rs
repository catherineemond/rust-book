fn main() {
    let mut value = 0;

    let result = loop {
        value += 1;
        if value == 10 {
            break value * 2;
        }
    };

    println!("The result is: {result}");
}
