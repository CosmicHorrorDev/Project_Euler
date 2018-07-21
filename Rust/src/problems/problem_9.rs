pub fn solution() -> String {
    let sum = 1000;
    let (a, b, c) = pythagorean_triplet(sum);
    let product = a * b * c;
    format!("The pythogorean triplet where a, b, & c sum to 1000 is {}, {}, \
& {} with a product of {}", a, b, c, product)
}


fn pythagorean_triplet(upper: u32) -> (u32, u32, u32) {
    let mut a = 1;
    let mut b = 1;
    let mut c = upper - 2;
    while c > a {
        let mut found = false;
        while c > b {
            found = is_triple(a, b, c);
            if found {
                break;
            }

            c -= 1;
            b += 1;
        }
        if found {
            break;
        }

        a += 1;
        b = a;
        c = upper - a - b;
    }

    (a, b, c)
}

fn is_triple(a: u32, b: u32, c: u32) -> bool {
    (a *  a) + (b * b) == (c * c)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triple() {
        assert_eq!(is_triple(3, 4, 5), true);
        assert_eq!(is_triple(3, 3, 3), false);
    }

    #[test]
    fn test_pythagorean_triplet() {
        assert_eq!(pythagorean_triplet(12), (3, 4, 5));
    }
}
