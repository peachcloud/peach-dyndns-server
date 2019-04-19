# peach-dyndns-host

a DNS server to host the names of guests with changing IP addresses

_work in progress_

## demo

```shell
git clone git@github.com:peachcloud/peach-dyndns-host
cd peach-dyndns-host
cargo run
```

in another terminal

```shell
dig @localhost -p 12323 test.dyn.peach.cloud
```
