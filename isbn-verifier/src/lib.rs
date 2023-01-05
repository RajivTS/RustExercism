/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace('-', "");
    if !isbn.is_ascii() || isbn.len() != 10 {
        return false;
    }
    let (mut sum, mut multiplier) = (0, 10);
    for (idx, &elem) in isbn.as_bytes().into_iter().enumerate() {
        let num = (elem as char)
            .to_digit(10)
            .or(if elem == b'X' && idx == isbn.len() - 1 { Some(10) } else { None });
        match num {
            Some(num) => {
                sum += num * multiplier;
                multiplier -= 1;
            },
            None => return false,
        }
    }
    sum % 11 == 0
}
