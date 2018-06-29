fn main() {
    let bound = 999;

    let mut num1 = bound;
    let mut num2 = bound;
    let mut slide1;
    let mut slide2;

    // There will be a palindrome once it hits 9 for sure so no need for cond.
    loop {
        slide1 = num1;
        slide2 = num2;

        let mut palindrome = false;
        // Slide down the diagonal, checking for palindromes
        while slide1 < bound + 1 && slide2 > 0 {
            palindrome = is_palindrome(&split(&mut (slide1 * slide2)));
            if palindrome {
                break;
            }
            slide1 += 1;
            slide2 -= 1;
        }
        // Cascade the break out of the loops
        if palindrome {
            break;
        }

        if num1 > num2 {
            // Move right one block
            num1 -= 1;
        } else {
            // Move down one block
            num2 -= 1;
        }
    }

    println!("{} = {} * {}", slide1 * slide2, slide1, slide2);
}


// Splits a number into a Vec of the digits
// could try making it into a u8 to see if that is faster
fn split(num: &mut u32) -> Vec<u32> {
    let mut digits = Vec::new();

    while *num > 0 {
        // Num but the ones place is 0 e.g. num is 1234, stripped is 1230
        let stripped = *num / 10 * 10;

        // Store ones place
        let digit = *num - stripped;
        digits.push(digit);

        // Rinse & Repeat
        *num = stripped / 10;
    }

    digits.reverse();
    digits
}


// Compares a slice to it's reverse to see if it's a palindrome
fn is_palindrome<T>(v: &[T]) -> bool
where
    T: Eq,
{
    v.iter().eq(v.iter().rev())
}
