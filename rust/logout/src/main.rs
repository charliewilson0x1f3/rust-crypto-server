use std::
{
	collections::HashMap,
	env,
	fs,
	path::Path
};

use auth::
{
	redirect,
	valid_cookie
};

fn remove_cookie(username : String)
{
	if Path::new(&("cookies/".to_string()+&username)).exists()
	{
		fs::remove_file("cookies/".to_string()+&username).expect("Failed to delete file...");
	}
}

fn main()
{
	let path = Path::new("/var/www/server");
        assert!(env::set_current_dir(&path).is_ok()); 

        let mut cookie = "".to_string();
        // extract cookie
	let env_vars: HashMap<String, String> = std::env::vars().collect();
        for (key, value) in env_vars.clone()
        {
                if key == "HTTP_COOKIE"
                {
                	cookie = value;
                }
        }
	let username = valid_cookie(cookie);
	if username != "ERROR".to_string()
	{
		remove_cookie(username);
		redirect("../login.html".to_string());	
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
