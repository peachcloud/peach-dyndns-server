#!/bin/bash

MYIP="1.1.1.44"

KEY=green.dyn.commoninternet.net.key
NS=ns.commoninternet.net
DOMAIN=green.dyn.commoninternet.net.
ZONE=green.dyn.commoninternet.net

nsupdate -k $KEY -v << EOF
server $NS
zone $ZONE
update delete $DOMAIN A
update add $DOMAIN 30 A $MYIP
send
EOF
