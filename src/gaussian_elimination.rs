// http://rosettacode.org/wiki/Gaussian_elimination

use std::num::Zero;
use std::ops::*;
extern crate core;
use core::num::Float;
//extern crate num;
//use num::traits::Num;

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


trait GteAbs {
    fn gteabs(&self, other: &Self) -> bool;
}


impl GteAbs for f64{
    fn gteabs(&self, other: &Self) -> bool {
        return self.abs() >= *other
    }
}


impl<'a, T> Iterator for MatrixRowIter<'a, T> where T: Copy {
    type Item = (T, usize);
    fn next(&mut self) -> Option<(T, usize)> {
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
        Some((v, ret))
    }
}


//Div<Output=T> + Mul<Output=T>
fn gaussian_elimination<T>(a : &[T], b : &[T], r : &mut [T]) where T: Copy + GteAbs + core::num::Float {

    let mut used: [bool; MAT_SIZE] = [false; MAT_SIZE];

    let mut bm: [T; MAT_SIZE] = unsafe{std::mem::uninitialized()};

    let mut a_mod: [T; MAT_SIZE*MAT_SIZE] = unsafe{std::mem::uninitialized()};

    for i in 0..MAT_SIZE {
        let max = {
            let mat_iter = MatrixRowIter::new(&a, MAT_SIZE, &mut used, i);
            mat_iter.fold((T::zero(), -1), |max, item| if item.0.gteabs(&max.0) { item } else { max } )
        };

        used[max.1] = true;

        for (i, o) in get_row_slice(&a, MAT_SIZE, max.1).iter()
                                                        .zip(a_mod[ i * MAT_SIZE .. (i+1) * MAT_SIZE ]
                                                        .iter_mut()) {
            *o = i.clone();
        }

        bm[i] = b[max.1];
    }

    let mut mat_out = Matrix::new(&mut a_mod, MAT_SIZE);

    for dia in 0..mat_out.n {
        for row in dia+1..mat_out.n {
            let tmp : T = *mat_out.val(Idx(dia, row)) / *mat_out.val(Idx(dia, dia));
            for col in dia+1..mat_out.n {
                *mat_out.val(Idx(col, row)) -= tmp * *mat_out.val(Idx(col, dia));
            }
            *mat_out.val(Idx(dia, row)) = 0.0;
            bm[row] -= tmp * bm[dia];
        }
    }


    for row in (0 .. MAT_SIZE).rev() {
        let mut tmp = bm[row];
        for j in (row+1 .. MAT_SIZE).rev() {
            tmp -= r[j] * *mat_out.val(Idx(j, row));
        }
        r[row] = tmp / *mat_out.val(Idx(row, row));
    }
}


fn main() {

    let a : [f64; MAT_SIZE*MAT_SIZE] = [
        1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
        1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
        1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
        1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
        1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
        1.00, 3.14, 9.87, 31.01, 97.41, 306.02  ];

    let b: [f64; MAT_SIZE] = [-0.01, 0.61, 0.91, 0.99, 0.60, 0.02];
    let mut r: [f64; MAT_SIZE] = unsafe{std::mem::uninitialized()};


    gaussian_elimination(a, b, r);

    for j in 0..MAT_SIZE {
        print!("{:9.5} ", r[j]);
    }
    println!("");

}

