use nn::Matrix;

fn main() {
	let m1 = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
	let mut m2 = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
	m2 *= m1;

	println!("{m2:?}");
}
