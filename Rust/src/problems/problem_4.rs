pub fn solution() -> String {
    let upper = 1000;
    let (num1, num2) = largest_palindrome_product(upper);
    let product = num1 * num2;
    format!("The largest palindrome product is {} = {} * {}",
            product, num1, num2)
}


fn largest_palindrome_product(upper: usize) -> (usize, usize) {
    // nums move down the table by moving down then right, slides move
    // diagonally down and left
    let mut num1 = upper - 1;
    let mut num2 = num1;
    let mut slide1;
    let mut slide2;

    // Loop should never fail, since single digits are all palindromes
    loop {
        slide1 = num1;
        slide2 = num2;

        let mut palindrome = false;
        // The condition represents the edge of the table
        while slide1 < upper && slide2 > 0 {
            palindrome = is_palindrome(slide1 * slide2);
            if palindrome {
                break;
            }
            // Move down-left
            slide1 += 1;
            slide2 -= 1;
        }
        //TODO: try using a named loop so that the break doesn't have to be
        // cascaded, may save some cycles
        // Cascades the break
        if palindrome {
            break;
        }

        if num1 > num2 {
            // Move right
            num1 -= 1;
        } else {
            // Move down
            num2 -= 1;
        }
    }

    (slide1, slide2)
}


fn is_palindrome(num: usize) -> bool {
    let mut num_compare = num as isize;
    let mut slide = 1;
    while slide < num_compare {
        slide *= 10;
    }
    slide /= 10;

    while num_compare > 9 {
        let stripped = num_compare / 10 * 10;
        let digit = num_compare - stripped;
        num_compare -= digit * slide;

        if num_compare >= slide || num_compare <= -1 {
            return false;
        }

        slide /= 100;
        num_compare /= 10;
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_palindrome_product() {
        assert_eq!(largest_palindrome_product(100), (99, 91));
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(9009), true);
        assert_eq!(is_palindrome(12321), true);
        assert_eq!(is_palindrome(1231), false);
        assert_eq!(is_palindrome(3112), false);
        assert_eq!(is_palindrome(1000), false);
    }
}
