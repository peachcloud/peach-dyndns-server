# peach-dyndns-host

a dynamic DNS server to host the names of guests with changing IP addresses. provides an http API 
for updating bind9 configurations. 

_work in progress_

## demo

```shell
git clone git@github.com:peachcloud/peach-dyndns-host
cd peach-dyndns-host
cargo run -- -vvv # DEBUG log verbosity
```

in another terminal

```shell
nslookup blue.dyn.peachcloud.org ns.peachcloud.org 
```

or

```shell
curl http://localhost:3000
```


## testing

contains bash scripts for testing and debugging dynamic dns server behavior using nslookup