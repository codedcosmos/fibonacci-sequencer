// What should I have called it large? Sizable? Considerable?
// HUGE, IMMENSE, ENORMOUS, MASSIVE, COLOSSAL, GARGANTUAN, MONOLITHIC
// yeah nah, biguint, let's keep it simple
// I get that this int is really, really really big like, u4294967296 big
// But like c'mon to much effort to type anything else
// Yes I get the irony in writing this massive thing
// But anyways thanks for reading the source code
// -codedcosmos

use std::fmt;

pub const BIT_STORE_SIZE: usize = 10;

#[derive(Debug)]
struct BitStore {
	array: [u8; BIT_STORE_SIZE],
}

#[derive(Debug)]
pub struct BigInt {
	values: Vec<Box<BitStore>>,
}

#[derive(Debug)]
pub struct TextBuffer {
	a: Vec<Box<BitStore>>,
	b: Vec<Box<BitStore>>,

	bitstore_index: usize,
	removing_leading_zeros: bool,
}

impl BitStore {
	pub fn new() -> Self {
		BitStore {
			array: [0; BIT_STORE_SIZE]
		}
	}
}

impl BigInt {
	pub fn new() -> Self {
		BigInt {
			values: vec![Box::new(BitStore::new())],
		}
	}

	pub fn allocate_more(&mut self, capacity: usize) {
		for _i in 0..capacity {
			self.values.push(Box::new(BitStore::new()));
		}
	}

	pub fn ensure_capacity(&mut self, capacity: usize) {
		let diff = abs_diff(capacity, self.values.len());
		self.allocate_more(diff);
	}

	pub fn one() -> Self {
		let mut bigint = BigInt::new();

		bigint.values[0].array[0] = 0b1000_0000;

		bigint
	}

	pub fn set_to(&mut self, other: &BigInt) {
		for index in 0..other.values.len() {
			loop {
				// Get Self bit store
				match self.values.get_mut(index) {
					None => {
						self.allocate_more(1);
					}
					Some(self_bit_store) => {
						// Get other bit store
						match other.values.get(index) {
							None => {
								panic!("Could not grab expected value whilst calling set_to on bigint!");
							}
							Some(other_bit_store) => {
								self_bit_store.array = other_bit_store.array.clone();
							}
						}

						break;
					}
				}
			}
		}
	}

	pub fn set_as_sum(&mut self, a: &mut BigInt, b: &mut BigInt) {
		let mut carry_bit = false;

		let capacity = a.values.len().max(b.values.len()).max(self.values.len());
		a.ensure_capacity(capacity);
		b.ensure_capacity(capacity);
		self.ensure_capacity(capacity);

		// Iterate over arrays
		let mut array_index: usize = 0;
		loop {
			// Escape if entirely iterated
			if array_index == a.values.len() || array_index == b.values.len(){
				break;
			}

			// Get arrays
			let a = &a.values[array_index].array;
			let b = &b.values[array_index].array;
			let c = &mut self.values[array_index].array;

			// Iterate through bytes
			let mut byte_index: usize = 0;
			loop {
				if byte_index == a.len() || byte_index == b.len(){
					break;
				}

				// Get bytes
				let a = a[byte_index];
				let b = b[byte_index];

				// Iterate through bits
				let mut bit_index: usize = 0;
				loop {
					if bit_index == 8 {
						break;
					}

					// Do math
					let mut sum: u8 = 0;
					if a & (1 << 7-bit_index) != 0 {
						sum += 1;
					}
					if b & (1 << 7-bit_index) != 0 {
						sum += 1;
					}
					if carry_bit {
						sum += 1;
					}

					match sum {
						0 => {
							// Set 0
							c[byte_index] &= !(1 << (7-bit_index));

							// Don't Carry bit
							carry_bit = false;
						}
						1 => {
							// Set 1
							c[byte_index] |= 1 << (7-bit_index);

							// Don't Carry bit
							carry_bit = false;
						}
						2 => {
							// Set 0
							c[byte_index] &= !(1 << (7-bit_index));

							// Carry bit
							carry_bit = true;
						}
						3 => {
							// Set 1
							c[byte_index] |= 1 << (7-bit_index);

							// Carry bit
							carry_bit = true;
						}
						_ => {
							panic!("Invalid sum value of {} at index's {} {} {}", sum, array_index, byte_index, bit_index);
						}
					}

					// Increment
					bit_index += 1;
				}

				// Increment
				byte_index += 1;
			}

			// Increment
			array_index += 1;
		}

		// Increase capacity if needed
		if carry_bit {
			a.allocate_more(1);
			b.allocate_more(1);
			self.allocate_more(1);

			let index = self.values.len()-1;
			self.values[index].array[0] |= 1 << (7-0);
		}
	}
}

impl TextBuffer {
	pub fn new() -> Self {
		TextBuffer {
			a: vec![Box::from(BitStore::new())],
			b: vec![Box::from(BitStore::new())],

			bitstore_index: 0,
			removing_leading_zeros: true,
		}
	}

	pub fn get_decimal(&mut self, big_int: &BigInt) {
		// Set A to all 0s
		for bit_store in self.a.iter_mut() {
			for byte in bit_store.array.iter_mut() {
				*byte = 0;
			}
		}

		// Set first bit to 1
		*self.a.first_mut().unwrap().array.first_mut().unwrap() = 1;

		// Set B to all 0s
		for bit_store in self.b.iter_mut() {
			for byte in bit_store.array.iter_mut() {
				*byte = 0;
			}
		}

		// Iterate over arrays
		for bit_store in big_int.values.iter() {
			let i = bit_store.array;

			// Iterate through bytes
			for byte in i.iter() {

				// Iterate through bits
				let mut bit_index: usize = 0;
				loop {
					if bit_index == 8 {
						break;
					}

					// If bit is set,
					// if it is:
					// 	add a to b
					if byte & (1 << 7-bit_index) != 0 {
						let mut a_iter = self.a.iter();
						let mut b_iter = self.b.iter_mut();
						loop {
							match (a_iter.next(), b_iter.next()) {
								(Some(a), Some(b)) => {
									// Get arrays
									let a = &a.array;
									let b = &mut b.array;

									// Iterate through bytes
									for byte_index in 0..BIT_STORE_SIZE {
										b[byte_index] += a[byte_index];
									}
								}
								(_, _) => {
									break;
								}
							}
						}


						// Calculate b overflow
						let mut carry_bit = false;
						for bit_store in self.b.iter_mut() {
							for byte in bit_store.array.iter_mut() {
								if carry_bit {
									*byte += 1;
									carry_bit = false;
								}
								if *byte >= 10 {
									*byte -= 10;
									carry_bit = true;
								}
							}
						}

						// Increase size if run out of storage for b
						if carry_bit {
							let mut bit_store = BitStore::new();
							*bit_store.array.first_mut().unwrap() = 1;
							self.b.push(Box::from(bit_store));

							self.a.push(Box::from(BitStore::new()));
						}
					}

					// Double a
					for bit_store in self.a.iter_mut() {
						for byte in bit_store.array.iter_mut() {
							*byte *= 2;
						}
					}

					// Calculate a overflow
					let mut carry_bit = false;
					for bit_store in self.a.iter_mut() {
						for byte in bit_store.array.iter_mut() {
							if carry_bit {
								*byte += 1;
								carry_bit = false;
							}
							if *byte >= 10 {
								*byte -= 10;
								carry_bit = true;
							}
						}
					}

					// Increase size if run out storage for a
					if carry_bit {
						let mut bit_store = BitStore::new();
						*bit_store.array.first_mut().unwrap() = 1;
						self.a.push(Box::from(bit_store));

						self.b.push(Box::from(BitStore::new()));
					}

					// Increment
					bit_index += 1;
				}
			}
		}

		self.bitstore_index = 0;
		self.removing_leading_zeros = true;
	}
}

impl Iterator for TextBuffer {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		// If bit store index is to high then return none as there is nothing left
		if self.bitstore_index >= self.b.len() {
			return None;
		}

		// Get array (in reverse order)
		let bit_store = self.b.get(self.b.len()-self.bitstore_index-1).unwrap();

		// Create string
		let mut return_text = String::with_capacity(BIT_STORE_SIZE);

		// iterate over bytes (in reverse order)
		for byte in bit_store.array.iter().rev() {
			if *byte != 0 {
				self.removing_leading_zeros = false;
			}

			if !self.removing_leading_zeros {
				return_text.push_str(byte.to_string().as_str());
			}
		}

		self.bitstore_index += 1;

		Some(return_text)
	}
}

impl fmt::Display for BigInt {
	// This trait requires `fmt` with this exact signature.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut text = String::new();
		for i in self.values.iter() {
			for o in i.array.iter() {
				text.push_str(&*format!("_{:#010b}", o))
			}
		}

		write!(f, "BigInt {}", text)
	}
}

fn abs_diff(x: usize, y: usize) -> usize {
	if x < y {
		y - x
	} else {
		x - y
	}
}