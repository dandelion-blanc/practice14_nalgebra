
extern crate nalgebra;


fn main()
{
	let mat = nalgebra::DMatrix::from_vec(5, 4, vec![
																	1.0_f64,	3.0_f64,	6.0_f64,	1.0_f64,
																	2.0_f64,	4.0_f64,	1.0_f64,	3.0_f64,
																	7.0_f64,	3.0_f64,	2.0_f64,	6.0_f64,
																	3.0_f64,	1.0_f64,	4.0_f64,	1.0_f64,
																	9.0_f64,	2.0_f64,	1.0_f64,	3.0_f64,
																]
													);
	let source_vec = vec![2.0_f64, 1.0_f64, 3.0_f64, 1.0_f64];

	let vec_ = nalgebra::DVector::from_vec(source_vec.clone());					// for console output
	assert_eq!(mat.ncols(), source_vec.len());
	let vec = source_vec.clone().into_iter().cycle().take( mat.nrows()*mat.ncols() ).collect::<Vec<f64>>();
																						// broadcast
	let vec = nalgebra::DMatrix::from_row_slice(mat.nrows(), mat.ncols(), &vec);
																						// change vec to matrix
	println!("mat{ }\n{ }", mat, vec_.transpose());

//	let prod = mat.dot(&vec);
//	let prod = vec.component_mul(&mat);
	let prod = mat.component_mul(&vec);
	println!("{ }", prod);

	println!("-----------------------------------------------");
	
	let mut prod = mat.clone();
	prod.row_iter_mut()
		.for_each(
			|mut row|
				row.iter_mut().zip(source_vec.iter()).for_each(|(x, v)| *x *= v)
		);
	println!("{ }", prod);

}