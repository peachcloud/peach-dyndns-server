# peach-dyndns-host

a dynamic DNS server to host the names of guests with changing IP addresses
by providing an http API for updating bind9 configurations. 


## setup 

The code in this repo assumes the existence of an installed and running bind9 server on the same 
server as is running peach-dyndns-server. Documentation for setting up bind9 can be found [here](docs/setup-bind-for-peach-dyndns.md).

The peach-dyndns-server code can be compiled with
```
cargo build --release
```

## run

```
sudo su peach-dyndns; ./target/release/main -vv
```

## test

test peach-dyndns server is running,
```
curl http://localhost:8000
```

test peach-bind9 is running,
```
nslookup blue.dyn.peachcloud.org ns.peachcloud.org
```
