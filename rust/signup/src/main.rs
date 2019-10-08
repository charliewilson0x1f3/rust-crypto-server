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
use unicode::unicode;

fn signup(mut username: String, mut password1 : String, mut password2 : String) -> bool
{
	let uname_len = username.len();
	let pwd1_len = password1.len();

	if str::replace(&username.clone(), "%", "") != username
	{
		username = unicode(username);
		if username == "ERROR".to_string()
		{
			return false;
		}
	}
	if str::replace(&password1.clone(), "%", "") != password1
	{
		password1 = unicode(password1);
		password2 = unicode(password2);
		if password1 == "ERROR".to_string() ||
		   password2 == "ERROR".to_string()
		{
			return false;
		}
	}
	
	if username == "admin".to_string()
	{
		return false;
	}	
	if uname_len > 16 || uname_len < 1
	{
		return false;
	}
	if pwd1_len < 8
	{
		return false;
	}
	if password1 != password2
	{
		return false;
	}
	for c in username.chars()
	{
		if !c.is_alphanumeric()
		{
			return false;
		}
	}
	let mut pwd_upper = false;
	let mut pwd_lower = false;
	let mut pwd_num = false;
	let mut pwd_spec = false;
	for c in password1.chars()
	{
		if c.is_uppercase(){pwd_upper = true}
		if c.is_lowercase(){pwd_lower = true}
		if c.is_numeric(){pwd_num = true}
		if !c.is_alphanumeric(){pwd_spec = true}
	}
	if !(pwd_upper && pwd_lower && pwd_num && pwd_spec)
	{
		return false;
	}

	let paths = fs::read_dir("./passwords").unwrap();

	for path in paths
	{
		let filename: String = path.unwrap().path().display().to_string();
		let spath: Vec<&str> = filename.split("/").collect();
		if spath.len() != 3
		{
			return false;
		}
		if spath[2].to_string() == *username
		{
			return false;
		}
	}

	let mut hasher = Sha512::new();
	hasher.input(password1.as_bytes());
	let hashed_pass = hasher.result();
	
	fs::write("requests/".to_string()+&username, hashed_pass).expect("Unable to write file (signup request)");
	return true;
}

fn main()
{
	let path = Path::new("/var/www/server");
	assert!(env::set_current_dir(&path).is_ok()); 

	let mut query = "".to_string();
	// extract query string
	let env_vars: HashMap<String, String> = std::env::vars().collect();
        for (key, value) in env_vars.clone()
	{
		if key == "QUERY_STRING"
		{
			query = value;
		}
	}

	let queries: Vec<&str> = query.split("&").collect();

	let mut username = "".to_string();
	let mut password1 = "".to_string();
	let mut password2 = "".to_string();
	let mut valid = false;
	
	if queries.len() == 3
	{
		let username_vec: Vec<&str> = queries[0].split("=").collect();
		let password1_vec: Vec<&str> = queries[1].split("=").collect();
		let password2_vec: Vec<&str> = queries[2].split("=").collect();
		if username_vec.len() == 2 && password1_vec.len() == 2 && password2_vec.len() == 2
		{
			if username_vec[0] == "username" && password1_vec[0] == "password1" && password2_vec[0] == "password2"
			{
				username = username_vec[1].to_string();
				password1 = password1_vec[1].to_string();
				password2 = password2_vec[1].to_string();
				valid = true;
			}
		}
	}

	let mut link = "../signup-error.html";

	if valid // sign up
	{
		if signup(username, password1, password2)
		{
			link = "../signup-success.html";
		}
	}
	redirect(link.to_string());
}
