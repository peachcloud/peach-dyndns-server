# peach-dyndns-host

a dynamic DNS server to host the names of guests with changing IP addresses
by providing an http API for updating bind9 configurations. 


## Setup

The code in this repo assumes the existence of an installed and running bind9 server on the same 
server as is running peach-dyndns-server. Documentation for setting up bind9 can be found [here](docs/setup-bind-for-peach-dyndns.md).

The peach-dyndns-server code can be compiled with
```
cargo deb; sudo dpkg -i target/debian/peach-dyndns-server_0.1.0_amd64.deb
```

## Development

```
sudo su peach-dyndns; ./target/release/main -vv
```

## Prod Deployment 

prod is deployed to /srv/peachcloud/peach-dyndns-server/prod-peach-dyndns

## Staging Deployment 

staging is deployed to /srv/peachcloud/peach-dyndns-server/dev-peach-dyndns

## Test

test peach-dyndns server is running,
```
curl http://localhost:8000
```

test peach-bind9 is running,
```
nslookup blue.dyn.peachcloud.org ns.peachcloud.org
```
