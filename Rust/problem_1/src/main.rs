fn main() {
    let upperbound = 1000;
    let mut sum = 0;

    for i in 1..upperbound {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("Sum of multiples of 3 and 5 below {} is {}", upperbound, sum);
}
