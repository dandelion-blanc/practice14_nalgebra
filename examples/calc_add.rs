extern crate nalgebra;


fn main()
{
	let mat1 = nalgebra::DMatrix::from_row_slice(4, 4, &vec![
																			7.0_f64,	3.0_f64,	2.0_f64,	6.0_f64,
																			3.0_f64,	1.0_f64,	4.0_f64,	1.0_f64,
																			2.0_f64,	4.0_f64,	1.0_f64,	3.0_f64,
																			9.0_f64,	2.0_f64,	1.0_f64,	3.0_f64,
																		]
															);
	let mat2 = nalgebra::DMatrix::from_row_slice(4, 4, &vec![
																			1.0_f64,	3.0_f64,	6.0_f64,	1.0_f64,
																			2.0_f64,	4.0_f64,	1.0_f64,	3.0_f64,
																			1.0_f64,	3.0_f64,	6.0_f64,	1.0_f64,
																			3.0_f64,	1.0_f64,	4.0_f64,	1.0_f64,
																		]
															);

	let result = &mat1 + &mat2;
	println!("mat1{ }\nmat2{ }\nadd result{ }", mat1, mat2, result);

}