extern crate crypto;
//extern crate rsa;

use crypto::
{
	blockmodes::NoPadding,
	aes::
	{
		cbc_decryptor,
		KeySize::KeySize256
	},
	buffer::
	{
		RefReadBuffer,
		RefWriteBuffer
	}
};
//use rsa::PaddingScheme;
use std::
{
	collections::HashMap,
	env,
	fs,
	io::
	{
		Read,
		stdin
	},
	path::Path,
	process::Command,
	thread,
	time
};

use auth::
{
	redirect,
	valid_cookie
};
use file_to_bytes::file_to_bytes;
//use load_private_key::get_rsa_key;
use unicode::unicode;

fn upload(packet : Vec<u8>, username : String, filename : String, filetype : bool, original_len : usize) -> bool
{
	let max = 50000000;// 50 MB max file size
	
	if filename.len() > 64 || filename.len() == 0 || username.len() == 0
	{
		return false;
	}

	if packet.len() as i64 > max
	{
		return false;
	}
	
	let mut path = "keys/".to_string()+&username+"/";
	if filetype
	{
		path = "files/".to_string()+&username+"/";
	}

	if !Path::new(&path).exists()
	{
		fs::create_dir(&path).expect("Unable to create dir");
	}
	if Path::new(&(path.clone()+&filename)).exists() && filetype
	{
		return false;
	}
	
	if filetype // need to decrypt
	{
		// wait for iv and key
		while !(Path::new(&("keys/".to_string()+&username+"/"+&filename+".iv")).exists() &&
		        Path::new(&("keys/".to_string()+&username+"/"+&filename+".iv")).exists())
		{
			let time = time::Duration::from_secs(1);
			thread::sleep(time);
		}
			

		let iv = file_to_bytes("keys/".to_string()+&username+"/"+&filename+".iv");
		let /*cipher*/key = file_to_bytes("keys/".to_string()+&username+"/"+&filename+".key");
		let mut file = vec![0; original_len];

		// OAEP not implemented for this library...
		//let rsakey = get_rsa_key();
		//let key = rsakey.decrypt(PaddingScheme::OAEP, &cipherkey).expect("failed to decrypt");

		let command = "rm keys/".to_string()+&username+"/"+&filename+"*";
		Command::new("sh")
        		.arg("-c")
        		.arg(&command)
			.output()
        		.expect("failed to execute process");

		let mut aes = cbc_decryptor(KeySize256, &key, &iv, NoPadding);
		let mut file_buf = RefWriteBuffer::new(&mut file);
		let _result = aes.decrypt(&mut RefReadBuffer::new(&packet), &mut file_buf, true).unwrap();

		fs::write(path.clone()+&filename, file).expect("Unable to write file");
		return true;
	}
	else
	{
		fs::write(path.clone()+&filename, packet).expect("Unable to write file");
		return true;
	}
}

fn response(success : bool)
{
	let mut link = "../upload-error.html".to_string();
	if success
	{
		link = "../upload-success.html".to_string();
	}
	println!("Content-type: text/html");
        println!("");
	println!("{}", link);
}


fn main()
{
	let path = Path::new("/var/www/server");
        assert!(env::set_current_dir(&path).is_ok()); 

        let mut cookie = "".to_string();
	let mut filename = "".to_string();
	let mut filetype = false;
	let mut original_len : usize = 0;
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
		if key == "HTTP_FILETYPE"
		{
			if value.clone() == "file"
			{
				filetype = true;
			}
		}
		if key == "HTTP_OLENGTH"
		{
			original_len = value.parse().unwrap();
		}
        }

	let content_length: usize = env_vars.get("CONTENT_LENGTH").and_then(|cl| cl.parse::<usize>().ok()).unwrap_or(0);
	let mut request = vec![0; content_length];
	stdin().read_exact(&mut request).unwrap();

	let username = valid_cookie(cookie);
	filename = unicode(filename);
	if username != "ERROR".to_string()
	{
		response(upload(request, username, filename, filetype, original_len));
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
