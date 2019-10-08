use std::
{
	collections::hash_map::HashMap,
	env,
	path::Path,
	process::Command
};

use auth::
{
	redirect,
	valid_cookie
};
use unicode::unicode;

fn lock_user(username : String)
{
	let command = "mv passwords/".to_string() + &username + " requests/";
	Command::new("sh")
        	.arg("-c")
        	.arg(&command)
        	.output()
        	.expect("failed to lock user...");
}

fn main()
{
	let path = Path::new("/var/www/server");
        assert!(env::set_current_dir(&path).is_ok()); 

        let mut cookie = "".to_string();
	let mut query = "".to_string();
        // extract cookie and query
	let env_vars: HashMap<String, String> = std::env::vars().collect();
        for (key, value) in env_vars.clone()
        {
                if key == "HTTP_COOKIE"
                {
                	cookie = value.clone();
                }
		if key == "QUERY_STRING"
                {
                        query = value.clone();
                }
        }
        let mut username = "".to_string();
        
	let username_vec: Vec<&str> = query.split("=").collect();
        if username_vec.len() == 2
        {
        	if username_vec[0] == "username"
                {
                        username = username_vec[1].to_string();
                }
        }

	username = unicode(username);

	let mut valid = true;
        for c in username.chars()
        {
                if !c.is_alphanumeric()
                {
                        valid = false;
                }
        }

	if valid_cookie(cookie) == "admin".to_string()
	{
		if username != "".to_string() && valid
		{
			lock_user(username);
		}
		redirect("start.cgi".to_string());
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
