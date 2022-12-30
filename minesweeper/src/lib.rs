pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for row_idx in 0..minefield.len() {
        let mut chars = Vec::new();
        for col_idx in 0..minefield[row_idx].as_bytes().len() {
            let curr_pos = elem_at(minefield, row_idx as i32, col_idx as i32);
            match curr_pos {
                Some(b'*') => chars.push('*'),
                Some(b' ') => {
                    let pos = surrounding_pos(minefield, row_idx as i32, col_idx as i32);
                    let num = pos.into_iter().filter_map(std::convert::identity).filter(|&&e| e == b'*').count() as u32;
                    if num > 0 {
                        chars.push(char::from_digit(num, 10).unwrap());
                    } else {
                        chars.push(' ');
                    }
                },
                _ => continue
            }
        }
        result.push(chars.iter().collect::<String>());
    }
    result
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}


fn elem_at<'a>(minefield: &[&'a str], row_idx: i32, col_idx: i32) -> Option<&'a u8> {
    if row_idx < 0 || col_idx < 0 {
        None
    } else {
        minefield.get(row_idx as usize).and_then(|row| row.as_bytes().get(col_idx as usize))
    }
}

fn surrounding_pos<'a>(minefield: &[&'a str], row_idx: i32, col_idx: i32) -> Vec<Option<&'a u8>> {
    let left = elem_at(minefield, row_idx, col_idx - 1);
    let top_left = elem_at(minefield, row_idx - 1, col_idx - 1);
    let top = elem_at(minefield, row_idx - 1, col_idx);
    let top_right = elem_at(minefield, row_idx - 1, col_idx + 1);
    let right = elem_at(minefield, row_idx, col_idx + 1);
    let bottom_right = elem_at(minefield, row_idx + 1, col_idx + 1);
    let bottom = elem_at(minefield, row_idx + 1, col_idx);
    let bottom_left = elem_at(minefield, row_idx + 1, col_idx - 1);
    vec![left, top_left, top, top_right, right, bottom_right, bottom, bottom_left]
}