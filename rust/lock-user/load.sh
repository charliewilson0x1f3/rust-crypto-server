cargo build
sudo cp target/debug/lock-user /var/www/cgi-bin/lock-user.cgi
sudo chmod 755 /var/www/cgi-bin/*
