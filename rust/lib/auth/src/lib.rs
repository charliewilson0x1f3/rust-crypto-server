use std::fs;
use std::time::Duration;

pub fn valid_cookie(cookie: String) -> String
{
        // see if cookie exists
        let paths = fs::read_dir("./cookies").unwrap();

        for p in paths
        {
                let path = p.unwrap();
                let filename: String = path.path().display().to_string();
                let real_cookie = fs::read_to_string(&filename).expect("Unable to read file (cookie)");
                if cookie == real_cookie
                {
                        // set expiration to 30 minutes
                        let expiration = Duration::from_secs(1800);
                        let mtime = path.path().metadata().unwrap().modified().unwrap();
                        if mtime.elapsed().unwrap() > expiration
                        {
                                return "ERROR".to_string();
                        }
                        let spath: Vec<&str> = filename.split("/").collect();
                        if spath.len() == 3
                        {
                                return spath[2].to_string();
                        }
                }
        }
	return "ERROR".to_string();
}

pub fn redirect(link : String)
{
        println!("Content-type: text/html");
        println!("");
        println!("
<!DOCTYPE html>
<html>
        <head>
                <title>Unauthorized...</title>
                <link rel=\"stylesheet\" href=\"/css/style.css\">
                <meta charset=\"UTF-8\">
        </head>
        <body bgcolor=\"black\">
        <script>
                window.location.href = \"{}\";
        </script>
        </body>
</html>", link);
}
