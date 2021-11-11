//! 線形代数ライブラリの練習
//!
//!

extern crate nalgebra;

use std::f64::consts;

fn main()
{
	let double_pi = 2.0_f64 * consts::PI;
	let vec_x = nalgebra::DVector::from_fn(10, |x, _| 1.3_f64*(double_pi * x as f64 / 20.0_f64).sin() );
	println!("x:\n{ }", vec_x);

	let mat_a = nalgebra::DMatrix::
					from_fn(10, 10,
						|r, c| (3.0_f64*r as f64 - 0.7_f64*c as f64)
									*( (-1_i64).pow(r as u32 * c as u32) as f64) );
	println!("A:\n{ }", mat_a);

	let result = vec_x.transpose()*mat_a;
	println!("x*A:\n{ }", result.transpose());


}