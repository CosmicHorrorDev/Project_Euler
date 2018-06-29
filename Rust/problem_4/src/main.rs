fn main() {
    let mut num1 = 999;
    let mut num2 = num1;
    let mut slide1 = num1;
    let mut slide2 = num1;

    while num2 > 0 {
        slide1 = num1;
        slide2 = num2;

        let mut palindrome = false;
        while slide1 < 1000 && slide2 > 0 {
            palindrome = is_palindrome(&split(&mut (slide1 * slide2)));
            if palindrome {
                break;
            }
            slide1 += 1;
            slide2 -= 1;
        }
        if palindrome {
            break;
        }

        if num1 > num2 {
            num1 -= 1;
        } else {
            num2 -= 1;
        }
    }

    println!("{} = {} * {}", slide1 * slide2, slide1, slide2);
}


fn split(num: &mut u32) -> Vec<u32> {
    let mut digits = Vec::new();

    while *num > 0 {
        let stripped = *num / 10 * 10;

        let digit = *num - stripped;
        digits.push(digit);

        *num = stripped / 10;
    }
    digits.reverse();
    digits
}


fn is_palindrome<T>(v: &[T]) -> bool
where
    T: Eq,
{
    v.iter().eq(v.iter().rev())
}
