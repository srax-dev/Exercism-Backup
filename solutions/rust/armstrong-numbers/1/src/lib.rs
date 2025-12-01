pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut working_num = num;
    let mut size = num.to_string().len();
    while size > 1 {
        let pow: u32 = 10_u32.pow(size as u32 - 1);
        digits.push(working_num / pow);
        working_num = working_num % pow;
        size -= 1;
    }
    digits.push(working_num);
    let mut sum = 0;
    let digit_len = digits.len() as u32;
    for digit in digits {
        sum += digit.pow(digit_len);
    }
    if sum == num { true } else { false }
}