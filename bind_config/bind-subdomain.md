


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

For each subdomain, 
- generate a new ddns key (tsig-keygen -a hmac-md5 {{subdomain}}.dyn.commoninternet.net) and append it to /etc/bind/dyn.commoninternet.net.keys
- add a zone section to named.conf.local, associating the key with the subdomain  [B]
- add a zone file to /var/lib/bind/subdomain.dyn.commoninternet.net [C]
- reload bind and return the secret key to the client 

Add the following to /var/lib/bind/{{subdomain}}.dyn.commoninternet.net: [C]
```
$ORIGIN      .
$TTL 30      ; 30 seconds
{{subdomain}}.dyn.commoninternet.net              IN SOA  ns.commoninternet.net. root.commoninternet.net. (
                             2016062801 ; serial
                             3600       ; refresh (1 hour)
                             600        ; retry (10 minutes)
                             2600       ; expire (43 minutes 20 seconds)
                             30         ; minimum (30 seconds)
                             )
                     NS      ns.commoninternet.net.
```

Append the following to /etc/bind/named.conf.local: [B]
```
zone "{{subdomain}}.dyn.commoninternet.net" {
type master;
file "/var/lib/bind/{{subdomain}}.dyn.commoninternet.net";
     update-policy {
	grant {{subdomain}}.dyn.commoninternet.net self {{subdomain}}.dyn.commoninternet.net;
     };
};
```


Questions:
- an easy way to delete a subdomain? 
