//extern crate rsa;
//extern crate rand;

//use rand::rngs::OsRng;
//use rsa::{PublicKey, PaddingScheme};
use sha2::
{
	Sha512,
	Digest
};
use std::
{
	collections::HashMap,
	env,
	io::
	{
		stdout,
		Write
	},
	path::Path
};

use auth::
{
	redirect,
	valid_cookie
};
use file_to_bytes::file_to_bytes;
//use load_private_key::get_rsa_key;

fn download(filename : String, username : String)
{
	let path = "files/".to_string() + &username + "/" + &filename;
	if Path::new(&path).exists()
	{
		let contents = file_to_bytes(path);
		let mut hasher = Sha512::new();
		hasher.input(contents);
		let hash = hasher.result();

		// OAEP not implemented for this library...
		//let mut rng = OsRng::new().expect("no secure randomness available");
		//let rsakey = get_rsa_key();
		//let sig = rsakey.encrypt(&mut rng, PaddingScheme::OAEP, &hash).expect("failed to decrypt");

		println!("Content-type: multipart/form-data");
        	println!("");
		// change hash to sig after RSA works
		stdout().write(&hash).expect("Error... Failed to write hash");
	}
	else
	{
		redirect("../download-error.html".to_string());
	}
}

fn main()
{
	let path = Path::new("/var/www/server");
        assert!(env::set_current_dir(&path).is_ok()); 

        let mut cookie = "".to_string();
	let mut filename = "".to_string();
        // extract cookie and filename
        let env_vars: HashMap<String, String> = std::env::vars().collect();
	for (key, value) in env_vars.clone()
        {
                if key == "HTTP_COOKIE"
                {
                	cookie = value.clone();
                }
		if key == "HTTP_FILENAME"
		{
			filename = value.clone();
		}
        }

	let username = valid_cookie(cookie);
	if username != "ERROR".to_string()
	{
		download(filename, username);
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
