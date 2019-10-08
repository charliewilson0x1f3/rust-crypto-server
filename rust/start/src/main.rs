use std::
{
	collections::HashMap,
	env,
	fs,
	path::Path
};

use auth::{redirect, valid_cookie};

fn admin()
{
	println!("Content-type: text/html");
        println!("");
	println!("
<!DOCTYPE html>
<html>
	<head>
		<title>Admin</title>
		<link rel=\"stylesheet\" href=\"/css/style.css\">
		<meta charset=\"UTF-8\">
	</head>
	<body bgcolor=\"black\">
	<center>");

	let upaths = fs::read_dir("./passwords").unwrap();

	let mut ulist = "\t<div>\n\t\t<h1>Active Users:</h1>\n".to_string();

    	for upath in upaths
	{
		let user = upath.unwrap().path().display().to_string();
		let uname: Vec<&str> = user.split("/").collect();
		if uname.len() == 3
		{
			if uname[2] != "admin"
			{
				ulist += &("\t\t<h3>".to_string() + uname[2]+ "\n");
				ulist += &("\t\t\t<a href=\"/cgi-bin/lock-user.cgi?username=".to_string() + &uname[2] + "\">Lock</a>\n");
				ulist += &("\t\t\t<a href=\"/cgi-bin/delete-user.cgi?username=".to_string() + &uname[2] + "\">Delete</a>\n");
				ulist += "\t\t</h3>\n";
			}
		}
	}
	ulist += "\t</div>";

	if ulist != "\t<div>\n\t\t<h1>Active Users:</h1>\n\t</div>".to_string()
	{
		println!("{}\n\t<br/>", ulist);
	}

	let rpaths = fs::read_dir("./requests").unwrap();

	let mut rlist = "\t<div>\n\t\t<h1>Inactive Users:</h1>\n".to_string();

    	for rpath in rpaths
	{
		let user = rpath.unwrap().path().display().to_string();
		let uname: Vec<&str> = user.split("/").collect();
		if uname.len() == 3
		{
			if uname[2] != "admin"
			{
				rlist += &("\t\t<h3>".to_string() + uname[2]+ "\n");
				rlist += &("\t\t\t<a href=\"/cgi-bin/activate-user.cgi?username=".to_string() + &uname[2] + "\">Activate</a>\n");
				rlist += &("\t\t\t<a href=\"/cgi-bin/delete-user-request.cgi?username=".to_string() + &uname[2] + "\">Delete</a>\n");
				rlist += "\t\t</h3>\n";
			}
		}
	}
	rlist += "\t</div>";

	if rlist != "\t<div>\n\t\t<h1>Inactive Users:</h1>\n\t</div>".to_string()
	{
		println!("{}\n\t\t<br/>", rlist);
	}

	println!("
		<a href=\"/cgi-bin/logout.cgi\">Log out</a>
	</center>
	</body>
</html>");
}

fn start(username : String)
{
	println!("Content-type: text/html");
        println!("");
	println!("
<!DOCTYPE html>
<html>
	<head>
		<title>Hello {}</title>
		<link rel=\"stylesheet\" href=\"/css/style.css\">
		<script src=\"https://code.jquery.com/jquery-1.10.2.js\"></script>
		<meta charset=\"UTF-8\">
	</head>
	<body bgcolor=\"black\">
	<script>
		function get_iv() {{
			const enc = new TextEncoder();
			const iv = enc.encode(\"s3curi+y_i5_c001\");
			return iv;
		}}

		function check_hash(hash1, hash2) {{
			if(hash1.byteLength != hash2.byteLength)
				return false;
			var dv1 = new Int8Array(hash1);
			var dv2 = new Int8Array(hash2);
			for (var i = 0; i < hash1.byteLength; i++)
			{{
				if(dv1[i] != dv2[i])
					return false;
			}}
			return true;
		}}

		function get_hash(filename) {{
			var xhr = new XMLHttpRequest();
			xhr.responseType = \"blob\";
			xhr.open(\"GET\", \"/cgi-bin/sig.cgi\");
			xhr.setRequestHeader(\"filename\", filename);
			xhr.responseType = \"arraybuffer\";
			xhr.send(\"\");
			xhr.onreadystatechange = function() {{
       				if(xhr.readyState == 4) {{
					return xhr.response;
				}}
			}}
		}}

		function download(filename) {{
			var xhr = new XMLHttpRequest();
			xhr.responseType = \"blob\";
			xhr.open(\"GET\", \"/cgi-bin/download.cgi\");
			xhr.setRequestHeader(\"filename\", filename);
			xhr.responseType = \"arraybuffer\";
			xhr.send(\"\");
			xhr.onreadystatechange = function() {{
       				if(xhr.readyState == 4) {{
					var password = prompt(\"Enter your decryption passphrase.\");
					if(password != null) {{
						//var blob = new Blob([xhr.response]);
						var ciphertext = xhr.response;
						const enc = new TextEncoder();
						const password_data = enc.encode(password);
						window.crypto.subtle.digest(\"SHA-256\", password_data)
						.then(function(digest) {{
						  window.crypto.subtle.importKey(
						  \"raw\",
						  digest,
						  \"AES-CTR\",
						  false,
						  [\"decrypt\", \"encrypt\"]).then(function(key) {{
						  // common iv for decryption
						  common_iv = get_iv();
						  // decrypt file with personal key
						    window.crypto.subtle.decrypt(
						    {{
						      name: \"AES-CTR\",
						      counter: common_iv,
						      length: 64
						    }},
						    key,
						    ciphertext).then(function(decrypted) {{
						      // change ciphertext -> decrypted when decrypt works
						      window.crypto.subtle.digest(\"SHA-512\", ciphertext)
						      .then(function(hash1) {{
						        var xhr = new XMLHttpRequest();
						        xhr.open(\"GET\", \"/cgi-bin/sig.cgi\");
						        xhr.setRequestHeader(\"filename\", filename);
						        xhr.responseType = \"arraybuffer\";
						        xhr.send(\"\");
						        xhr.onreadystatechange = function() {{
       						          if(xhr.readyState == 4) {{
						            var hash2 = xhr.response;
						            if(!check_hash(hash1, hash2))
						            {{
						              alert(\"File does not match hash!!\");
						              return;
						            }}
							    var blob = new Blob([decrypted], {{type : \"multipart/form-data\"}});
						            var url = URL.createObjectURL(blob);
						            var element = document.createElement('a');
						            element.setAttribute('href', url);
						            element.setAttribute('download', filename);
						            element.style.display = 'none';
						            document.body.appendChild(element);
						            element.click();
						            document.body.removeChild(element);
						          }}
						        }}
						      }});
						    }}).catch(function(err) {{
						      alert(err);
						    }});
						  }});
						}});
					}}
				}}
			}}
		}}
	</script>
	<center>
	<div>
		<a href=\"/cgi-bin/upload-file.cgi\">Upload a file</a>
	</div>
	<br/>", username);

	let path = "./files/".to_string() + &username;
	if Path::new(&path).exists()
	{
		let fpaths = fs::read_dir(&path).unwrap();
	
		let mut flist = "\t<div>\n\t\t<h1>Files:</h1>\n".to_string();

    		for fpath in fpaths
		{
			let file = fpath.unwrap().path().display().to_string();
			let fname: Vec<&str> = file.split("/").collect();
			if fname.len() == 4
			{
				flist += &("\t\t<h3>".to_string() + &fname[3]+ "\n");
				flist += &("\t\t\t<input type=\"submit\" onclick=\"download('".to_string() + &fname[3] + "')\" value=\"Download\" />");
				flist += "\t\t</h3>\n";
			}
		}
		flist += "\t</div>";
		if flist != "\t<div>\n\t\t<h1>Files:</h1>\n\t</div>".to_string()
		{
			println!("{}\n\t\t<br/>", flist);
		}
	}
		
	println!("
		<a href=\"/cgi-bin/logout.cgi\">Log out</a>
	</center>
	</body>
</html>");
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
	if username == "admin".to_string()
	{
		admin();
	}
	else if username != "ERROR".to_string()
	{
		start(username);
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
