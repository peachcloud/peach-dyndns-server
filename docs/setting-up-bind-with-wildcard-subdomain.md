This file contains notes which may be relevant for setting up bind, 
and are not directly necessary for running the code in this repository.


Add the following to /etc/bind/named.conf.local:
```
key "ddns-key.dyn.commoninternet.net" {
	algorithm hmac-sha256;
	secret "yoursecrethere";
};

zone "dyn.commoninternet.net" {
type master;
file "/var/lib/bind/dyn.commoninternet.net";
     update-policy {
	grant ddns-key.dyn.commoninternet.net subdomain dyn.commoninternet.net;
        };
};
```

Add the following to /var/lib/bind/dyn.commoninternet.net:
```
$ORIGIN      .
$TTL 30      ; 30 seconds
dyn.commoninternet.net              IN SOA  ns.commoninternet.net. root.commoninternet.net. (
                             2016062801 ; serial
                             3600       ; refresh (1 hour)
                             600        ; retry (10 minutes)
                             2600       ; expire (43 minutes 20 seconds)
                             30         ; minimum (30 seconds)
                             )
                     NS      ns.commoninternet.net.
```
Note that this file needs to be in /var/lib/bind for bind to have proper write permissions. 

You can then add, delete and update subdomains using nsupdate. 
