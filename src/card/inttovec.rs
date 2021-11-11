pub fn number_to_vec(n: u8) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n: u8 = n;
    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}
