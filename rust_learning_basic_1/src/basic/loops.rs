pub fn run() {
    let mut count = 0;

    // While loop
    while count < 5 {
        println!("While loop: Count = {}", count);
        count += 1;
    }

    // For loop
    for x in 0..5 {
        println!("For loop: X = {}", x);
    }
}
