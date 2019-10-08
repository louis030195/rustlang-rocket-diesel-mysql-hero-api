# rustlang-rocket-diesel-mysql-hero-api
based on [blog post](https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8) and the updated [version](https://github.com/clifinger/rustlang-rocket-diesel-mysql-hero-api) + instructions
<!-- diesel migration generate heroes -->

## Usage

### Mysql installation (At least in my case Ubuntu 19.04)
    sudo apt-get install libmysqlclient-dev # Needed or not?
    sudo apt-get install mysql-server
    sudo service mysql start
    # Changing password
    sudo mysql -u root -p
    use mysql;
    update user set authentication_string=PASSWORD("new_password") where user='root';
    update user set plugin="mysql_native_password";
    flush privileges;
    quit;
    sudo service mysql restart
### Run
    cargo install diesel_cli --no-default-features --features mysql
    export DATABASE_URL=mysql://root:root@localhost/heroes
    diesel setup
    diesel migration run
    cargo run
then

    curl -d '{"id":"1", "name":"a", "identity":"b", "hometown":"c", "age":1}' -H "Content-Type: application/json" -X POST http://localhost:8000/hero

if you want to reprint schema

    diesel print-schema > src/schema.rs
    
# Performances with [wrk](https://github.com/wg/wrk)

    sudo apt-get install build-essential libssl-dev git -y
    git clone https://github.com/wg/wrk.git wrk
    cd wrk
    make
    # move the executable to somewhere in your PATH, ex:
    sudo cp wrk /usr/local/bin
then

    cargo run &

then

    wrk -t2 -c5 -d5s --timeout 2s http://localhost:8000/heroes