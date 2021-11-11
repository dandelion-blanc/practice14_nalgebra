extern crate nalgebra;



fn main()
{
	let m = nalgebra::DMatrix::from_vec(2, 2, vec!	[
																	0.8,	1.1,
																	-0.4,	0.2
																]
													);
	println!("m:\n{ }", m);

	let inverse_m = &m.transpose();
	println!("inverse_m:\n{ }", inverse_m);

	let n = &m*inverse_m;
	println!("positive define matrix");
	println!("n (= m*inverse_m) :\n{ }", n);

	let eigen = n.symmetric_eigen();
	let e_vec = &eigen.eigenvectors;
	let e_value = &eigen.eigenvalues;
	println!("n-eigen vector :\n{ }", e_vec);
	println!("n-eigen value :\n{ }", e_value);



}