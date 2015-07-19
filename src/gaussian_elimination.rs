// http://rosettacode.org/wiki/Gaussian_elimination
#![allow(unused_variables)]
#![allow(dead_code)]



struct Matrix<'a, T:'a> where T: Copy {
	m : &'a mut Vec<T>,
	n : usize
}


struct Idx(usize, usize);

impl<'a, T:'a> Matrix<'a, T> where T: Copy {
	fn new<'b> (_m : &'b mut Vec<T>, _n : usize) -> Matrix<'b, T> {
		Matrix{m : /*&mut*/ _m, n : _n} 
	}

	fn val(&self, idx : Idx) -> & T
	{
		& self.m[idx.0 + idx.1 * self.n]
	}

	// fn val_mut<'b>(&'b mut self, idx : Idx) -> &'b mut T
	// {
	// 	let r : &mut T =	& mut (*self.m)[idx.0 + idx.1 * self.n];
	// 	r
	// }

	fn set(&mut self, idx : Idx, v : T)
	{
		self.m[idx.0 + idx.1 * self.n] = v;
	}

	fn get_row_slice(&self, row: usize) -> &[T]
	{
	  & self.m[row * self.n .. row * self.n + self.n ]
	}
}


struct MatrixRowIter<'a, T:'a> where T: Copy {
	m : &'a Matrix<'a, T>,
	row : usize,
	col : usize,
	used: &'a Vec<bool>
}

impl<'a, T> MatrixRowIter<'a, T> where T: Copy {
	fn new (_m : &'a Matrix<'a, T>, _used: &'a mut Vec<bool>, _col : usize) -> MatrixRowIter<'a, T> {
		MatrixRowIter{m : _m, row : 0, col : _col, used : _used } 
	}

}



impl<'a, T> Iterator for MatrixRowIter<'a, T> where T: Copy {
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

    	let v = *self.m.val(Idx(self.col, self.row));
    	let ret = self.row;
    	self.row += 1;
      Some((v, ret))
    }
}

fn main() {

	let mut v : Vec<f32> = vec![
		1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
		1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
		1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
		1.00, 1.88, 3.55,  6.70, 12.62, 230.80,
		1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
		1.00, 3.14, 9.87, 31.01, 97.41, 306.02];

	let mat = Matrix::new(&mut v, 6);

	let zero:f32 = 0.0;
	let mut r = Vec::new();
	let mut used: Vec<bool> = vec![false; mat.n];

	for i in 0..mat.n {

		let max;
		{
			let mat_iter = MatrixRowIter::new(&mat, &mut used, i);

			max = mat_iter
						.inspect(|&item| println!("{} {}", item.0, item.1) )
						.fold((0.0, -1), 
							|max, item| if item.0.abs() > max.0 { item } else { max } )
						;
		}

		used[max.1] = true;
		println!("max {} {}", max.0, max.1);
		println!("=======");

		r.extend(mat.get_row_slice(max.1).iter());
	}

	let mut mat_out = Matrix::new(&mut r, mat.n);
	for i in 0..mat_out.n {
		for j in 0..mat_out.n {
			print!("{} ", mat_out.val(Idx(j, i)));
		}
		println!("  ");
	}

	let n= mat_out.n;
	for dia in 0..n {
		for row in dia+1..n {
			//let tmp = *mat_out.val(Idx(dia, row)) / *mat_out.val(Idx(dia, dia));
			for col in dia+1..n {
				//(*mat_out.val(Idx(col, row))) -= tmp;// * *mat_out.val(Idx(col, dia));
				//mat_out.val_mut(Idx(dia, row)) = 0.0;
				//let ref mut  rr : &mut &f32 = mat_out.val_mut(Idx(dia, row));
//				let rr : & mut f32   = &mut mat_out.val_mut(Idx(dia, row));
	//			println!("{}", rr);
				//*mat_out.val_mut(Idx(dia, row)) = 0.0;
				mat_out.set(Idx(dia, row), &zero);
				//*rr = 0.0;
			}
		}	
	}

}
