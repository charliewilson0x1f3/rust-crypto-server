cargo build
sudo cp target/debug/upload /var/www/cgi-bin/upload.cgi
sudo chmod 755 /var/www/cgi-bin/*
