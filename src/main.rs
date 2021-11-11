//! 線形代数ライブラリの練習
//!
//!

extern crate nalgebra;



fn main()
{
	let m = nalgebra::Matrix3x4::new(	11, 12, 13, 14,
												21, 22, 23, 24,
												31, 32, 33, 34);
	println!("{ }", m);

	let m = nalgebra::DMatrix::from_diagonal_element(4, 4, 2.0);
	println!("{ }", m);

	let m = nalgebra::DMatrix::from_fn(3, 3, |r, c| (r * 2) as f64 + (c + 1) as f64);
	println!("m:\n{ }", m);

	let inverse_m = &m.transpose();
	println!("inverse_m:\n{ }", inverse_m);

	let n = &m*inverse_m;
	println!("n (= m*inverse_m) :\n{ }", n);

	let eigen = n.symmetric_eigen();
	let e_vec = &eigen.eigenvectors;
	let e_value = &eigen.eigenvalues;
	println!("n-eigen vector :\n{ }", e_vec);
	println!("n-eigen value :\n{ }", e_value);



}