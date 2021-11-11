
extern crate nalgebra;
extern crate num_complex;

use std::f64;
use std::f64::consts;
use num_complex::Complex;

fn main()
{
	let frames = 3;
	let m = nalgebra::DMatrix::from_fn(3, 3, |r, c|
													Complex{
																	re: (f64::cos(2.0*consts::PI * r as f64 * c as f64 / frames as f64)) as f64,
																	im: (-f64::sin(2.0*consts::PI * r as f64 * c as f64 / frames as f64)) as f64,
																}
												);
	println!("m:\n{ }", m);

	let m_conj = m.map( |x| x.conj() );
	println!("conj_m:\n{ }", m_conj);

	let n = nalgebra::DVector::from_vec( vec![
															Complex{re: 1.0_f64	, im: 2.0_f64},
															Complex{re: 3.0_f64	, im: 1.0_f64},
															Complex{re: 2.0_f64	, im: 4.0_f64},
														]
												);
	println!("n:\n{ }", n);

	let prod = m*n;											// prod = 列ベクトル
//	let prod = n.transpose()*m;							// prod = 行ベクトル
	println!("prod:\n{ }", prod);

	let comp = Complex{ re: 1.0_f64, im: -0.0_f64};	// 複素共役
	println!("{ } { }", comp, comp.conj());
}