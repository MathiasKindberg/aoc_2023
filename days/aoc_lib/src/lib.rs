// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// /// Two dimensional helper for quick row based and columnar iteration
// pub struct TwoDVec<T> {
//     data: Vec<T>,
//     row_len: usize,
// }

// impl<T> TwoDVec<T> {
//     pub fn new(input: Vec<Vec<T>>) -> Self {
//         assert!(!input.is_empty());
//         Self {
//             data: vec![],
//             row_len: input[0].len(),
//         }
//     }
// }
