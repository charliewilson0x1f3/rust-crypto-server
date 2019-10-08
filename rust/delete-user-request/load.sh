cargo build
sudo cp target/debug/delete-user-request /var/www/cgi-bin/delete-user-request.cgi
sudo chmod 755 /var/www/cgi-bin/*
