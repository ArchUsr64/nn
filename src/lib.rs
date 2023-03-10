#[derive(Clone)]
pub struct Matrix {
	elements: Vec<Vec<f32>>,
	row: usize,
	col: usize,
}

use std::fmt;
impl fmt::Debug for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let max_width = self
			.elements
			.iter()
			.flatten()
			.fold(0usize, |acc, elem| acc.max(format!("{}", elem).len()));

		let mut output = String::new();
		for i in 0..self.row {
			for j in 0..self.col {
				if j == 0 {
					output.push_str(&i.to_string());
					output.push('│');
				}
				let value = format!("{}", self.elements[i][j]);
				output.push_str(&" ".repeat(max_width - value.len()));
				output.push_str(&value);
				output.push('|');
			}
			output.push('\n');
		}

		let first_line = output.lines().rev().last().unwrap();
		let mut index_line = " ".repeat(first_line.len());
		index_line.push('\n');
		let mut col_count = 0;
		first_line.chars().enumerate().for_each(|(i, char)| {
			if char == '|' {
				index_line.replace_range(i..i + 1, &col_count.to_string());
				col_count += 1;
			}
		});
		output.insert_str(0, &index_line);

		write!(f, "{}", output)
	}
}

use std::ops;
impl ops::Add for Matrix {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		assert_eq!(self.row, rhs.row);
		assert_eq!(self.col, rhs.col);
		let mut elements = self.elements;
		for (i, row) in elements.iter_mut().enumerate() {
			for (j, element) in row.iter_mut().enumerate() {
				*element += rhs.elements[i][j];
			}
		}
		Self {
			row: self.row,
			col: self.col,
			elements,
		}
	}
}

impl ops::Neg for Matrix {
	type Output = Self;
	fn neg(self) -> Self::Output {
		let mut elements = self.elements;
		for row in elements.iter_mut() {
			for element in row.iter_mut() {
				*element *= -1.;
			}
		}
		Self {
			row: self.row,
			col: self.col,
			elements,
		}
	}
}

impl ops::Sub for Matrix {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		self + (-rhs)
	}
}

impl ops::Mul for Matrix {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		assert_eq!(self.col, rhs.row);
		let mut elements = vec![vec![0.; rhs.col]; self.row];
		for (i, row) in elements.iter_mut().enumerate() {
			for (j, element) in row.iter_mut().enumerate() {
				for k in 0..self.col {
					*element += self.elements[i][k] * rhs.elements[k][j];
				}
			}
		}
		Self {
			row: self.row,
			col: rhs.col,
			elements,
		}
	}
}

impl ops::AddAssign for Matrix {
	fn add_assign(&mut self, rhs: Self) {
		*self = self.clone() + rhs;
	}
}

impl ops::SubAssign for Matrix {
	fn sub_assign(&mut self, rhs: Self) {
		*self = self.clone() - rhs;
	}
}

impl ops::MulAssign for Matrix {
	fn mul_assign(&mut self, rhs: Self) {
		*self = self.clone() * rhs;
	}
}

use std::cmp;
impl cmp::PartialEq for Matrix {
	fn eq(&self, other: &Self) -> bool {
		if self.row != other.row || self.col != other.col {
			return false;
		}
		for i in 0..self.row {
			for j in 0..self.col {
				if (self.elements[i][j] - other.elements[i][j]).abs() > f32::EPSILON {
					return false;
				}
			}
		}
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let b = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
		let expected = Matrix::new(&[vec![6., 8.], vec![10., 12.]]);
		assert_eq!(a + b, expected);
	}

	#[test]
	#[should_panic]
	fn test_add_panic() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let b = Matrix::new(&[vec![5., 6.]]);
		let _result = a + b;
	}

	#[test]
	fn test_neg() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let expected = Matrix::new(&[vec![-1., -2.], vec![-3., -4.]]);
		assert_eq!(-a, expected);
	}

	#[test]
	fn test_sub() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let b = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
		let expected = Matrix::new(&[vec![-4., -4.], vec![-4., -4.]]);
		assert_eq!(a - b, expected);
	}

	#[test]
	#[should_panic]
	fn test_sub_panic() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let b = Matrix::new(&[vec![5., 6.]]);
		let _result = a - b;
	}

	#[test]
	fn test_mul() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let b = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
		let expected = Matrix::new(&[vec![19., 22.], vec![43., 50.]]);
		assert_eq!(a * b, expected);
	}

	#[test]
	#[should_panic]
	fn test_mul_panic() {
		let a = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let b = Matrix::new(&[vec![5., 6.]]);
		let _result = a * b;
	}

	#[test]
	fn test_add_assign() {
		let m1 = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let mut m2 = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
		m2 += m1;
		assert_eq!(m2, Matrix::new(&[vec![6., 8.], vec![10., 12.],]));
	}

	#[test]
	fn test_sub_assign() {
		let m1 = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let mut m2 = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
		m2 -= m1;
		assert_eq!(m2, Matrix::new(&[vec![4., 4.], vec![4., 4.],]));
	}

	#[test]
	fn test_mul_assign() {
		let m1 = Matrix::new(&[vec![1., 2.], vec![3., 4.]]);
		let mut m2 = Matrix::new(&[vec![5., 6.], vec![7., 8.]]);
		m2 *= m1;
		assert_eq!(m2, Matrix::new(&[vec![23., 34.], vec![31., 46.],]));
	}
}

impl Matrix {
	pub fn new(elements: &[Vec<f32>]) -> Self {
		Self {
			elements: elements.to_vec(),
			row: elements.len(),
			col: elements[0].len(),
		}
	}
}
