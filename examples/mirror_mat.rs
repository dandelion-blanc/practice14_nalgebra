
extern crate nalgebra;




fn main()
{
	let mut mat =
    nalgebra::DMatrix::from_row_slice(5, 4, &vec![
        1.0_f64,	3.0_f64,	6.0_f64,	1.0_f64,
        2.0_f64,	4.0_f64,	1.0_f64,	3.0_f64,
        7.0_f64,	3.0_f64,	2.0_f64,	6.0_f64,
        3.0_f64,	1.0_f64,	4.0_f64,	1.0_f64,
        9.0_f64,	2.0_f64,	1.0_f64,	3.0_f64,
    ]);

    let (mut left, mut right) = 
    mat.columns_range_pair_mut(0..mat.ncols() / 2, mat.ncols() / 2..mat.ncols());
    right.row_iter_mut()
        .zip(left.row_iter())
        .for_each(
            |(mut nr, pr)|
            {
                let ipr = pr.iter().map(|x| *x).collect::<Vec<f64>>();
                let ipr = ipr.iter().rev();
                nr.iter_mut()
                    .zip(ipr)
                    .for_each(|(n, p)| *n = *p)
            }
        );
    println!("{ }{ }", left, right);
    println!("{ }", mat);

	let mut mat =
    nalgebra::DMatrix::from_row_slice(5, 6, &vec![
        1.0_f64,	3.0_f64,	6.0_f64,	1.0_f64,    2.0_f64,    3.2_f64,
        2.0_f64,	4.0_f64,	1.0_f64,	3.0_f64,    4.4_f64,    8.0_f64,
        7.0_f64,	3.0_f64,	2.0_f64,	6.0_f64,    1.0_f64,    5.4_f64,
        3.0_f64,	1.0_f64,	4.0_f64,	1.0_f64,    6.0_f64,    7.0_f64,
        9.0_f64,	2.0_f64,	1.0_f64,	3.0_f64,    3.2_f64,    5.5_f64,
    ]);

    let (mut left, mut right) = 
    mat.columns_range_pair_mut(0..mat.ncols() / 2, mat.ncols() / 2..mat.ncols());
    right.row_iter_mut()
        .zip(left.row_iter())
        .for_each(
            |(mut nr, pr)|
            {
                let ipr = pr.iter().map(|x| *x).collect::<Vec<f64>>();
                let ipr = ipr.iter().rev();
                nr.iter_mut()
                    .zip(ipr)
                    .for_each(|(n, p)| *n = *p)
            }
        );
    println!("{ }", mat);
}