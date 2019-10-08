use std::
{
	collections::HashMap,
	env,
	path::Path
};

use auth::
{
	redirect,
	valid_cookie
};

fn response()
{
	println!("Content-type: text/html");
        println!("");
	println!("
<!DOCTYPE html>
<html>
	<head>
		<title>Upload</title>
		<link rel=\"stylesheet\" href=\"/css/style.css\">
		<meta charset=\"UTF-8\">
	</head>
	<body bgcolor=\"black\">
	<div id=\"loading\" style=\"display:none\"></div>
	<script>

function show_password() {{
  var pwd = document.getElementById(\"password\");
  if (pwd.type === \"password\") {{
    pwd.type = \"text\";
  }}
  else {{
    pwd.type = \"password\";
  }}
}}

function get_iv() {{
  const enc = new TextEncoder();
  const iv = enc.encode(\"s3curi+y_i5_c001\");
  return iv;
}}

function str2ab(str) {{
  const buf = new ArrayBuffer(str.length);
  const bufView = new Uint8Array(buf);
  for (let i = 0, strLen = str.length; i < strLen; i++) {{
    bufView[i] = str.charCodeAt(i);
  }}
  return buf;
}}

function process_file() {{
  document.getElementById(\"loading\").style.display = \"block\";
  var file = document.getElementById(\"upload-file\").files[0];
  if(file) {{
    var reader = new FileReader();
    reader.readAsArrayBuffer(file);
    reader.onload = function (evt) {{
      // create AES key
      window.crypto.subtle.generateKey(
      {{
        name: \"AES-CBC\",
        length: 256,
      }},
      true,
      [\"encrypt\", \"decrypt\"]).then(function(key) {{
        const exported = window.crypto.subtle.exportKey(\"raw\", key).then(function(key_buf) {{

// need to fix RSA on server side first
/*
          // get public key
          var xhr_pubkey = new XMLHttpRequest();
          xhr_pubkey.open(\"POST\", \"/cgi-bin/pub-key.cgi\");
          xhr_pubkey.responseType = \"text\";
          xhr_pubkey.send(\"\");
          xhr_pubkey.onreadystatechange = function() {{
            if(xhr_pubkey.readyState == 4) {{
              var pem = xhr_pubkey.response;
              const pemHeader = \"-----BEGIN PUBLIC KEY-----\";
              const pemFooter = \"-----END PUBLIC KEY-----\";
              const pemContents = pem.substring(pemHeader.length, pem.length - pemFooter.length - 1);
              const pemContents2 = pem2.substring(pemHeader.length, pem2.length - pemFooter.length);
              const binaryDerString = window.atob(pemContents);
              const binaryDer = str2ab(binaryDerString);
              window.crypto.subtle.importKey(
              \"spki\",
              binaryDer,
              {{
                name: \"RSA-OAEP\",
                hash: \"SHA-256\"
              }},
              true,
              [\"encrypt\"]).then(function(pub_key) {{
                // encrypt shared key
                window.crypto.subtle.encrypt(
                {{
                  name: \"RSA-OAEP\"
                }},
                pub_key,
                key_buf).then(function(cipherkey) {{
*/
          // send (not encrypted yet) AES key
          var xhr_key = new XMLHttpRequest();
          xhr_key.open(\"POST\", \"/cgi-bin/upload.cgi\");
          xhr_key.setRequestHeader(\"filename\", file.name + \".key\");
          xhr_key.setRequestHeader(\"filetype\", \"key\");
          // change key_buf to cipherkey when RSA is implemented
          xhr_key.send(key_buf);
          xhr_key.onreadystatechange = function() {{
            if(xhr_key.readyState == 4) {{
              if(xhr_key.responseText.includes(\"upload-success.html\")) {{
                var contents = evt.target.result;
                iv = window.crypto.getRandomValues(new Uint8Array(16));

                // send iv
                var xhr_iv = new XMLHttpRequest();
                xhr_iv.open(\"POST\", \"/cgi-bin/upload.cgi\");
                xhr_iv.setRequestHeader(\"filename\", file.name + \".iv\");
                xhr_iv.setRequestHeader(\"filetype\", \"key\");
                xhr_iv.send(iv);
                xhr_iv.onreadystatechange = function() {{
                  if(xhr_iv.readyState == 4) {{
                    if(xhr_iv.responseText.includes(\"upload-success.html\")) {{
                      // generate personal key from password
                      var password = document.getElementById(\"password\").value;
                      const enc = new TextEncoder();
                      const password_data = enc.encode(password);
                      window.crypto.subtle.digest(\"SHA-256\", password_data).then(function(digest) {{
                        window.crypto.subtle.importKey(
                        \"raw\",
                        digest,
                        \"AES-CTR\",
                        true,
                        [\"encrypt\", \"decrypt\"]).then(function(personal_key) {{
                          // common iv for decryption
                          common_iv = get_iv();
                          // encrypt file with personal key
                          window.crypto.subtle.encrypt(
                          {{
                            name: \"AES-CTR\",
                            counter: common_iv,
                            length: 64
                          }},
                          personal_key,
                          contents).then(function(encrypted) {{
                            // encrypt file with shared key
                            window.crypto.subtle.encrypt(
                            {{
                              name: \"AES-CBC\",
                              iv
                            }},
                            key,
                            // change contents -> encrypted when AES decrypt works on client side
                            encrypted).then(function(payload) {{
                              // send file
                              var xhr = new XMLHttpRequest();
                              xhr.open(\"POST\", \"/cgi-bin/upload.cgi\");
                              xhr.setRequestHeader(\"filename\", file.name);
                              xhr.setRequestHeader(\"filetype\", \"file\");
			      xhr.setRequestHeader(\"olength\", contents.byteLength);
                              xhr.send(payload);
                              xhr.onreadystatechange = function() {{
                                if(xhr.readyState == 4) {{
                                  window.location = xhr.responseText;
                                }}
                              }}
                            }});
                          }});
                        }});
                      }});
                    }}
                    else {{
                      document.getElementById(\"loading\").style.display = \"none\";
                      alert(\"ERROR: unable to upload file...\");
                    }}
                  }}
                }}
              }}
              else {{
                document.getElementById(\"loading\").style.display = \"none\";
                alert(\"ERROR: unable to upload file...\");
              }}
            }}
          }}
/*
                }});
              }});
            }}
          }}
*/
        }});
      }});
    }}
    reader.onerror = function(evt) {{
      document.getElementById(\"loading\").style.display = \"none\";
      alert(\"ERROR: unable to upload file...\");
    }}
  }}
  else {{
    document.getElementById(\"loading\").style.display = \"none\";
    alert(\"ERROR: unable to upload file...\");
  }}
}}
	</script>
	
	<center>
	<div>
		<form action=\"/cgi-bin/upload.cgi\" method=\"post\" enctype=\"multipart/form-data\">
		<form onsubmit=\"process_file()\">
			<p>Choose File: <input id=\"upload-file\" type=\"file\" name=\"file\" /></p>
		</form>
		<p>
			<input type=\"password\" id=\"password\" placeholder=\"encryption passphrase\" />
			<input type=\"checkbox\" onclick=\"show_password()\">Show
		</p>
		<p><input type=\"submit\" onclick=\"process_file()\" value=\"Upload\" /></p>
		<a href=\"/cgi-bin/start.cgi\">Back</a>
	</div>
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
	if username != "ERROR".to_string()
	{
		response();
	}
	else
	{
		redirect("../session-expired.html".to_string());
	}
}
