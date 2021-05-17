#!/bin/bash

MYIP="1.1.1.11"

KEY=ddns.key
NS=ns.commoninternet.net
DOMAIN=orange.time.commoninternet.net.
ZONE=time.commoninternet.net

nsupdate -k $KEY -v << EOF
server $NS
zone $ZONE
update delete $DOMAIN A
update add $DOMAIN 30 A $MYIP
send
EOF
