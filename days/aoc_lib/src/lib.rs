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

/// Reverses all vectors in 2d Vec
pub fn rev2<T>(v: &mut Vec<Vec<T>>) {
    for row in v {
        row.reverse()
    }
}

pub fn rotate_90_ccw_2d<T>(matrix: &mut Vec<Vec<T>>)
where
    T: Copy,
{
    let size = matrix.len();

    for x in 0..(size / 2) {
        for y in x..size - x - 1 {
            let temp = matrix[x][y];
            matrix[x][y] = matrix[y][size - 1 - x];
            matrix[y][size - 1 - x] = matrix[size - 1 - x][size - 1 - y];
            matrix[size - 1 - x][size - 1 - y] = matrix[size - 1 - y][x];
            matrix[size - 1 - y][x] = temp;
        }
    }
}

pub fn rotate_90_cw_2d<T>(matrix: &mut Vec<Vec<T>>)
where
    T: Copy,
{
    let size = matrix.len();

    // Traverse each cycle
    for i in 0..(size / 2) {
        for j in i..(size - i - 1) {
            // Swap elements of each cycle
            // in clockwise direction
            let temp = matrix[i][j];
            matrix[i][j] = matrix[size - 1 - j][i];
            matrix[size - 1 - j][i] = matrix[size - 1 - i][size - 1 - j];
            matrix[size - 1 - i][size - 1 - j] = matrix[j][size - 1 - i];
            matrix[j][size - 1 - i] = temp;
        }
    }
    // for (int i = 0; i < N / 2; i++) {
    //     for (int j = i; j < N - i - 1; j++) {

    //         // Swap elements of each cycle
    //         // in clockwise direction
    //         int temp = a[i][j];
    //         a[i][j] = a[N - 1 - j][i];
    //         a[N - 1 - j][i] = a[N - 1 - i][N - 1 - j];
    //         a[N - 1 - i][N - 1 - j] = a[j][N - 1 - i];
    //         a[j][N - 1 - i] = temp;
    //     }
    // }
}

pub fn print_2d<T>(v: &[Vec<T>])
where
    T: std::fmt::Display,
{
    for row in v {
        for c in row {
            print!("{c} ");
        }
        println!()
    }
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
