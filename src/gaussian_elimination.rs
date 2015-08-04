// http://rosettacode.org/wiki/Gaussian_elimination
use std::ops::*;

struct Idx(usize, usize);

const MAT_SIZE : usize = 6;


struct Matrix<'a, T:'a> where T:  'a + Copy {
    m : &'a mut [T],
    n : usize,
}


impl<'a, T:'a> Matrix<'a, T> where T: 'a + Copy {
    fn new<'b> (_m : &'b mut [T], _n : usize) -> Matrix<'b, T> {
        Matrix{m : _m, n : _n}
    }

    fn val<'b>(&'b mut self, idx : Idx) -> &'b mut T {
        & mut (*self.m)[idx.0 + idx.1 * self.n]
    }
}



fn get_row_slice<T>(m : &[T], n : usize, row: usize) -> &[T]  {
    & m[ row * n .. row * n + n ]
}

struct MatrixRowIter<'a, T:'a> where T: Copy {
    m : &'a [T],
    n : usize,
    row : usize,
    col : usize,
    used: &'a [bool]
}

impl<'a, T> MatrixRowIter<'a, T> where T: Copy {
    fn new (_m : &'a [T], _n: usize, _used: &'a mut [bool], _col : usize) -> MatrixRowIter<'a, T> {
        MatrixRowIter{m : _m, n: _n, row : 0, col : _col, used : _used }
    }
}



trait ValueType : Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {
    fn gteabs(&self, other: &Self) -> bool;
    fn zero() -> Self;
}


impl ValueType for f64{
    fn gteabs(&self, other: &Self) -> bool { return self.abs() >= *other  }
    fn zero() -> Self { return 0.0 }
}


impl<'a, T> Iterator for MatrixRowIter<'a, T> where T: Copy {
    type Item = (T, Option<usize>);
    fn next(&mut self) -> Option<(T, Option<usize>)> {
        loop {
            if self.row >= self.n {
                return None
            }

            if !self.used[self.row] {
                break;
            }

            self.row += 1;
        }

        let v = self.m[self.col + self.row * self.n];
        let ret = self.row;
        self.row += 1;
        Some((v, Some(ret)))
    }
}


fn gaussian_elimination<T>(a : &[T], b : &[T], r : &mut [T]) where T: Copy + ValueType {

    let mut used: [bool; MAT_SIZE] = [false; MAT_SIZE];

    let mut bm: [T; MAT_SIZE] = unsafe{std::mem::uninitialized()};

    let mut a_mod: [T; MAT_SIZE*MAT_SIZE] = unsafe{std::mem::uninitialized()};

    for i in 0..MAT_SIZE {
        let max = {
            let mat_iter = MatrixRowIter::new(&a, MAT_SIZE, &mut used, i);
            mat_iter.fold((T::zero(), None), |max, item| if item.0.gteabs(&max.0) { item } else { max } )
        };

        let max_index = max.1.unwrap();
        used[max_index] = true;

        for (i, o) in get_row_slice(&a, MAT_SIZE, max_index).iter()
                                                            .zip(a_mod[ i * MAT_SIZE .. (i+1) * MAT_SIZE ]
                                                            .iter_mut()) {
            *o = i.clone();
        }

        bm[i] = b[max_index];
    }

    let mut mat_out = Matrix::new(&mut a_mod, MAT_SIZE);

    for dia in 0..mat_out.n {
        for row in dia+1..mat_out.n {
            let tmp : T = *mat_out.val(Idx(dia, row)) / *mat_out.val(Idx(dia, dia));
            for col in dia+1..mat_out.n {
                *mat_out.val(Idx(col, row)) = *mat_out.val(Idx(col, row)) -
                                               tmp * *mat_out.val(Idx(col, dia));
            }
            *mat_out.val(Idx(dia, row)) = T::zero();
            bm[row] = bm[row] - tmp * bm[dia];
        }
    }


    for row in (0 .. MAT_SIZE).rev() {
        let mut tmp = bm[row];
        for j in (row+1 .. MAT_SIZE).rev() {
            tmp = tmp - r[j] * *mat_out.val(Idx(j, row));
        }
        r[row] = tmp / *mat_out.val(Idx(row, row));
    }
}


fn calculate_example(r : &mut [f64]) {

   let a : [f64; MAT_SIZE*MAT_SIZE] = [
        1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
        1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
        1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
        1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
        1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
        1.00, 3.14, 9.87, 31.01, 97.41, 306.02  ];

   let b: [f64; MAT_SIZE] = [-0.01, 0.61, 0.91, 0.99, 0.60, 0.02];

   gaussian_elimination(&a, &b, r);
}


#[cfg(not(test))]
fn main() {

    let mut r: [f64; MAT_SIZE] = unsafe{std::mem::uninitialized()};

    calculate_example(&mut r);

    for j in 0..MAT_SIZE {
        print!("{:9.5} ", r[j]);
    }
    println!("");
}



#[test]
fn test_result() {

    let expected_result: [f64; MAT_SIZE] = [-0.01000, 1.60279, -1.61320, 1.24549, -0.49099, 0.06576];
    let mut r: [f64; MAT_SIZE] = unsafe{std::mem::uninitialized()};

    calculate_example(&mut r);

    for (i, o) in expected_result.iter().zip(r.iter_mut()) {
        assert!((*i - *o).abs() < 0.001);
    }

}

