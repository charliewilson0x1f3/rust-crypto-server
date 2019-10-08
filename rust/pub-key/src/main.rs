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

fn get_key()
{
	let path = "RSA/key.pub".to_string();
	if Path::new(&path).exists()
	{
		let contents = file_to_bytes(path);
		
		println!("Content-type: multipart/form-data");
        	println!("");
		stdout().write(&contents).expect("Error... Failed to write file");
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
        // extract cookie and filename
        let env_vars: HashMap<String, String> = std::env::vars().collect();
	for (key, value) in env_vars.clone()
        {
                if key == "HTTP_COOKIE"
                {
                	cookie = value.clone();
                }
        }

	let username = valid_cookie(cookie);
	if username != "ERROR".to_string()
	{
		get_key();
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
