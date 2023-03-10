use nn::{Matrix, NN};

fn main() {
	let network = NN::new(&[2, 5, 1]);
	let result = network.forward_prop(Matrix::new(&[vec![1., 0.5]]));
	println!("{result:?}");
}
