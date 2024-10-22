extern crate rsa;

use std::process::Command;
use rsa::RSAPrivateKey;
use num_bigint::BigUint;

pub fn get_rsa_key() -> RSAPrivateKey
{
	let command = "openssl rsa -in RSA/key.pem -text -inform PEM -noout";
	let output = Command::new("sh")
		.arg("-c")
		.arg(&command)
		.output()
        	.expect("failed to execute process");
	let ostring = String::from_utf8_lossy(&output.stdout).to_string();

	let mut temp = "".to_string();
	let mut values = Vec::new();
	let mut i = 0;
	let mut prev_c = 'c';
	let mut iter = ostring.chars();
	for j in 0..ostring.len()
	{
		let c = iter.next().unwrap();

		// read to size
		if i == 0
		{
			if c == '('
			{
				i += 1;
			}
		}
		// get size
		else if i == 1
		{
			if c != ' '
			{
				temp += &c.to_string();
			}
			else
			{
				values.push(temp.clone());
				temp = "".to_string();
				i += 1;
			}
		}
		// read to modulus
		else if i == 2
		{
			if c == ':'
			{
				i += 1;
			}
		}
		// read modulus
		else if i == 3
		{
			if c != 'p'
			{
				if c != '\n' && c != ':' && c != ' '
				{
					temp += &c.to_string();
				}
			}
			else
			{
				values.push(temp.clone());
				temp = "".to_string();
				i += 1;
			}
		}
		// read to publicExponent
		else if i == 4
		{
			if c == ' '
			{
				i += 1;
			}
		}
		// read publicExponent
		else if i == 5
		{
			if c != ' '
			{
				temp += &c.to_string();
			}
			else
			{
				values.push(temp.clone());
				temp = "".to_string();
				i += 1;
			}
		}
		else if i >= 6
		{
			// read to _
			if i % 2 == 0
			{
				if c == ':'
				{
					i += 1;
				}
			}
			// get _
			else
			{
				if !(c.is_alphabetic() && prev_c == '\n')
				{
					if c != '\n' && c != ':' && c != ' '
					{
						temp += &c.to_string();
					}
					prev_c = c;
				}
				else
				{
					values.push(temp.clone());
					temp = "".to_string();
					i += 1;
					prev_c = 'c';
				}
				if j+1 == ostring.len()
				{
					values.push(temp.clone());
				}
			}
		}
	}
	// Key size
	let _key_size = values[0].clone().parse::<i32>().unwrap();
	// Modulus
	let n = BigUint::parse_bytes(values[1].clone().into_bytes().as_slice(), 16).unwrap();
	// Public exponent
	let e = BigUint::parse_bytes(values[2].clone().into_bytes().as_slice(), 10).unwrap();
	// Private exponent
	let d = BigUint::parse_bytes(values[3].clone().into_bytes().as_slice(), 16).unwrap();
	// Prime factors of N, contains >= 2 elements
	let prime1 = BigUint::parse_bytes(values[4].clone().into_bytes().as_slice(), 16).unwrap();
	let prime2 = BigUint::parse_bytes(values[5].clone().into_bytes().as_slice(), 16).unwrap();
	let primes : Vec<BigUint> = [prime1, prime2].to_vec();
	// exponents
	let _exponent1 = BigUint::parse_bytes(values[6].clone().into_bytes().as_slice(), 16).unwrap();
	let _exponent1 = BigUint::parse_bytes(values[7].clone().into_bytes().as_slice(), 16).unwrap();
	// coefficient
	let _coefficient = BigUint::parse_bytes(values[8].clone().into_bytes().as_slice(), 16).unwrap();

	return RSAPrivateKey::from_components(n, e, d, primes);
}
