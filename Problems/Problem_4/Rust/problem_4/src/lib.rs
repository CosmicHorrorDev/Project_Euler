pub fn largest_palindrome_product(upper: usize) -> (usize, usize) {
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
            palindrome = is_palindrome(&reverse_split(&mut (slide1 * slide2)));
            if palindrome {
                break;
            }
            // Move down-left
            slide1 += 1;
            slide2 -= 1;
        }
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


fn reverse_split(num: &mut usize) -> Vec<usize> {
    let mut digits = Vec::new();

    while *num > 0 {
        // Stripped is *num with 0 in the ones place
        let stripped = *num / 10 * 10;

        // Store ones place
        let digit = *num - stripped;
        digits.push(digit);

        // On to the next digit
        *num = stripped / 10;
    }
    digits
}


fn is_palindrome<T>(v: &[T]) -> bool
where
    T: Eq,
{
    v.iter().eq(v.iter().rev())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(largest_palindrome_product(100), (99, 91));
    }
}
