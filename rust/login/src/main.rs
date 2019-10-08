extern crate rand;
extern crate chrono;

use sha2::
{
	Sha512,
	Digest
};
use std::
{
	collections::HashMap,
	env,
	fs,
	path::Path
};

use auth::redirect;
use file_to_bytes::file_to_bytes;
use unicode::unicode;
 
fn authenticate(username: String, password : String) -> String
{
	let true_password = file_to_bytes("passwords/".to_string()+&username);
	let mut hasher = Sha512::new();
	hasher.input(password.as_bytes());
	let hashed_pass = hasher.result();
	if hashed_pass[..].to_vec() == true_password
	{
		let cookie1: i64 = rand::random();
		let cookie2: i64 = rand::random();
		let cookie3: i64 = rand::random();
		let cookie4: i64 = rand::random();
		let cookie = cookie1.to_string()+&cookie2.to_string()+&cookie3.to_string()+&cookie4.to_string();
	
		fs::write("cookies/".to_string()+&username, cookie.clone()).expect("Unable to write file (cookie)");
		return cookie;
	}

	return "".to_string();
}

fn main()
{
	let path = Path::new("/var/www/server");
	assert!(env::set_current_dir(&path).is_ok()); 

	let mut query = "".to_string();
	// extract query string
	let env_vars: HashMap<String, String> = std::env::vars().collect();
        for (key, value) in env_vars
	{
		if key == "QUERY_STRING"
		{
			query = value;
		}
	}

	let queries: Vec<&str> = query.split("&").collect();

	let mut username = "".to_string();
	let mut password = "".to_string();
	let mut valid = false;
	
	if queries.len() == 2
	{
		let username_vec: Vec<&str> = queries[0].split("=").collect();
		let password_vec: Vec<&str> = queries[1].split("=").collect();
		if username_vec.len() == 2 && password_vec.len() == 2
		{
			if username_vec[0] == "username" && password_vec[0] == "password"
			{
				username = username_vec[1].to_string();
				password = password_vec[1].to_string();
				valid = true;
			}
		}
	}

	username = unicode(username);
	password = unicode(password);

	let mut link = "../login-error.html";

	if valid // authenticate
	{
		
		let cookie = authenticate(username.clone(), password); 
		
		if cookie != "".to_string()
		{
			println!("set-cookie: {}", cookie);
			link = "start.cgi";
		}
	}
	redirect(link.to_string());
}
