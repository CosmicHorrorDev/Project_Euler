pub fn largest_prime_factor() {
    let mut number: u64 = 600851475143;
    let mut index = 3;

    // Factor out 2 seperately so I can index by 2 instead of 1
    while number % 2 == 0 {
        number /= 2;
    }

    loop {
        while number % index== 0 {
            number /= index;
        }

        // All values are factored out, index is the largest prime factor
        if number == 1 {
            break;
        }

        index += 2;
    }

    println!("The largest prime factor is {}", index);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
