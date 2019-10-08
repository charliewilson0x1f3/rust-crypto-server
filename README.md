# rust-crypto-server
Example of a secure file server in rust using cgi scripts

## About
There aren't many simple examples of Rust webservers out there, so I decided to make my own. Using the rust binaries as CGI scripts should produce a fairly responsive server. There are two types of user, admin and basic. There is only one admin. The admin approves signup requests and can remove or temporarily lock out (and unlock) a user. Basic users can upload and download files. Each file is encrypted/decrypted with a key generated from a password.

## Security
- Requires strong passwords (At least 8 characters (ASCII) and includes lowercase, uppercase, number, special character)
- Passwords are hashed with SHA512
- Session IDs are a concatenation of 4 pseudorandom 64 bit ints
- Files encrytped with AES256 (key is a SHA256 sum of a chosen password)
- Files exchanged with AES256 encryption, key exchanged with RSA
- Filenames and Usernames support Unicode

## Setup
This is run on Linux with the following dependencies:
- mv
- rm
- openssl
- cargo (rust)

The server implementation is up to you. I recommend Apache. Place the project in /var/www. In the directory "rust," cd into each subdirectory besides "lib" (lib is where custom shared libraries are) and run "./load.sh" (compiles rust and moves the executables to the cgi-bin - make sure cargo is in PATH). Each of those directories is an independent Rust project, and they all have proper "load" scripts. If needed, enable CGI scripts with the server, and point "/cgi-bin/" to "var/www/cgi-bin." Also, any file or directory needs correct permissions to be accessed by the server.
The administrator username is always "admin." I left the admin with "Security2@" as the password. You can change this by deleting "server/passwords/admin" and signing up as admin again. Just make sure you move the generated "server/requests/admin" to "server/passwords/admin."

## What this project currently needs
- I could not find any good rust libraries for RSA, so it is broken on my server. This means the symmetric key for data transfer is sent unencrypted. RSA support needs to be added to the server for it to work. The framework is there, and client side is ready to go (just uncomment). A quick fix could be to have a python script (or whatever chosen language has good RSA libraries) on the server side to handle the data transfer.
- Code cleanup... I wrote the code in a very short amount of time, so there may be some long one-liners, and the client-side code is a bit messy (promise chaining for the crypto subtle portions may help).
- Peer review. Security people, please tear this code to shreds if it makes you happy!
