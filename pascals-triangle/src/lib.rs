
pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

fn get_elem(input: &Vec<u32>, idx: i32) -> u32 {
    if idx < 0 {
        0
    } else {
        input.get(idx as usize).copied().unwrap_or(0)
    }
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Vec::with_capacity(row_count as usize);
        for row in 0..row_count {
            if row == 0 {
                triangle.push(vec![1]);
                continue;
            }
            let prev_row = triangle.get(row as usize - 1).unwrap();
            let mut curr_row = Vec::with_capacity(row as usize + 1);
            for col in 0..=row {
                let val = get_elem(prev_row, col as i32) + get_elem(prev_row, col as i32 - 1);
                curr_row.push(val);
            }
            triangle.push(curr_row);

        }
        Self {
            rows: triangle
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
