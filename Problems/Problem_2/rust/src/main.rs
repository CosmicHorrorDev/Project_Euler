fn main() {
    let upperbound = 4000000;

    let mut sum = 2;

    let mut previous = 1;
    let mut current = 2;

    while previous + current < upperbound + 1 {
        current += previous;
        previous = current - previous;

        if current % 2 == 0 {
            sum += current;
        }
    }

    println!("Sum of even Fibonacci numbers below {} is {}", upperbound, sum);
}
