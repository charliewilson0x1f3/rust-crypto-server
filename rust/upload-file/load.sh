cargo build
sudo cp target/debug/upload-file /var/www/cgi-bin/upload-file.cgi
sudo chmod 755 /var/www/cgi-bin/*
