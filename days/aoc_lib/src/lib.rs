/// Two dimensional helper for quick row based and columnar iteration
pub struct TwoDVec<T> {
    data: Vec<T>,
    row_len: usize,
}

impl<T> TwoDVec<T> {
    pub fn new(input: Vec<Vec<T>>) -> Self {
        assert!(!input.is_empty());
        Self {
            data: vec![],
            row_len: input[0].len(),
        }
    }
}
