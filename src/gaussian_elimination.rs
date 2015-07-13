// http://rosettacode.org/wiki/Gaussian_elimination
#![allow(unused_variables)]
#![allow(dead_code)]



struct Matrix<'a, T:'a> {
	m : &'a Vec<T>,
	n : usize
}

struct MatrixRowIter<'a, T:'a> {
	m : &'a Matrix<'a, T>,
	row : usize,
	col : usize
}

struct Idx(usize, usize);

impl<'a, T> Matrix<'a, T> {
	fn new (_m : &'a Vec<T>, _n : usize) -> Matrix<'a, T> {
		Matrix{m : _m, n : _n} 
	}

	fn val(&self, idx : Idx) -> &T
	{
		& self.m[idx.0 + idx.1 * self.n]
	}
}


impl<'a, T> MatrixRowIter<'a, T> {
	fn new (_m : &'a Matrix<'a, T>, _col : usize) -> MatrixRowIter<'a, T> {
		MatrixRowIter{m : _m, row : 0, col : _col} 
	}

}



impl<'a, T:Copy> Iterator for MatrixRowIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
    	if self.row >= self.m.n {
    	  return None
    	}
    	let v = *self.m.val(Idx(self.col, self.row));
    	self.row += 1;
      Some(v)
    }
}

fn main() {

	let v : Vec<f32> = vec![
		1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
		1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
		1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
		1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
		1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
		1.00, 3.14, 9.87, 31.01, 97.41, 306.02];

	let mat = Matrix::new(&v, 6);	
	let mat_iter = MatrixRowIter::new(&mat, 5);

	let mat_iter_print = &mat_iter;
	{
		for v in *mat_iter_print
		{
			println!("{}", v);
		}
	}

	let max = mat_iter.fold(0.0f32, |max, item| { max.max(item.abs()) } );
	println!("{}", max);
}
