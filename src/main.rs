use nn::NN;

fn main() {
	let network = NN::new(&[2, 4, 1]);
	let result = network.forward_prop(&[1., 0.5]);
	println!("{result:?}");
}
