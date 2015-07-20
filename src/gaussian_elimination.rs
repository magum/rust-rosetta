// http://rosettacode.org/wiki/Gaussian_elimination
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Display;

struct Idx(usize, usize);



struct Matrix<'a, T:'a> where T:  'a + Copy {
	m : &'a mut [T],
	n : usize,
}


impl<'a, T:'a> Matrix<'a, T> where T: 'a + Copy + Display {
	fn new<'b> (_m : &'b mut [T], _n : usize) -> Matrix<'b, T> {
		Matrix{m : _m, n : _n}
	}

	fn val(&self, idx : Idx) -> & T
	{
		& (*self.m)[idx.0 + idx.1 * self.n]
	}

	fn val_mut<'b>(&'b mut self, idx : Idx) -> &'b mut T
	{
		let r : &mut T =	& mut (*self.m)[idx.0 + idx.1 * self.n];
		r
	}

	fn get_row_slice(&self, row: usize) -> &[T]
	{
	  & self.m[row * self.n .. row * self.n + self.n ]
	}

	fn print(&self) {
		for i in 0..self.n {
			for j in 0..self.n {
				print!("{:9.3} ", self.val(Idx(j, i)));
			}
			println!("  ");
		}
	}
}


struct MatrixRowIter<'a, T:'a> where T: Copy {
	m : &'a Matrix<'a, T>,
	row : usize,
	col : usize,
	used: &'a [bool]
}

impl<'a, T> MatrixRowIter<'a, T> where T: Copy {
	fn new (_m : &'a Matrix<'a, T>, _used: &'a mut [bool], _col : usize) -> MatrixRowIter<'a, T> {
		MatrixRowIter{m : _m, row : 0, col : _col, used : _used }
	}
}



impl<'a, T> Iterator for MatrixRowIter<'a, T> where T: Copy + Display {
    type Item = (T, usize);
    fn next(&mut self) -> Option<(T, usize)> {
    	loop {
	    	if self.row >= self.m.n {
	    	  return None
	    	}
	    	if !self.used[self.row] {
	    		break;
	    	}
    		self.row += 1;
    	}

    	let v = self.m.val(Idx(self.col, self.row));
    	let ret = self.row;
    	self.row += 1;
      Some((*v, ret))
    }
}

fn main() {

	const MAT_SIZE : usize = 6;

	// let mut v : Vec<f32> = vec![
	// 	1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
	// 	1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
	// 	1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
	// 	1.00, 1.88, 3.55,  6.70, 12.62, 230.80,
	// 	1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
	// 	1.00, 3.14, 9.87, 31.01, 97.41, 306.02];

	let mut v : Vec<f32> = vec![
		1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
		1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
		1.00, 1.88, 3.55,  6.70, 12.62, 230.80,
		1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
		1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
		1.00, 3.14, 9.87, 31.01, 97.41, 306.02];


	let mat = Matrix::new(&mut v, MAT_SIZE);

	let mut r : Vec<f32> = Vec::new();
	let mut used: [bool; MAT_SIZE] = [false; MAT_SIZE];

	for i in 0..mat.n {

		let max;
		{
			let mat_iter = MatrixRowIter::new(&mat, &mut used, i);

			max = mat_iter
						.inspect(|&item| println!("{:9.3} {:5}", item.0, item.1) )
						.fold((0.0, -1),
							|max, item| if item.0.abs() >= max.0 { item } else { max } )
						;
		}

		println!("max v:{:9.3} idx:{}", max.0, max.1);
		used[max.1] = true;
		println!("=======");

		r.extend(mat.get_row_slice(max.1).iter().cloned());
	}

	let mut mat_out = Matrix::new(&mut r, mat.n);
	mat_out.print();

	println!("========================");

	for dia in 0..mat_out.n {
		for row in dia+1..mat_out.n {
			let tmp = *mat_out.val(Idx(dia, row)) / *mat_out.val(Idx(dia, dia));
			for col in dia+1..mat_out.n {
				*mat_out.val_mut(Idx(col, row)) -= tmp * *mat_out.val(Idx(col, dia));
				*mat_out.val_mut(Idx(dia, row)) = 0.0;
			}
		}
	}

	mat_out.print();

}

