#!/usr/bin/env bash

ECHO=$(which echo)
NSUPDATE=$(which nsupdate)

# Set the DNS entry you want to update, please notice the final dot.
HOST="test.dyn.commoninternet.net"

# Set the key provided by your DNS administrator
KEY="/etc/named/Kmydomain.com.+157+19553.key"

# Set the DNS server name or IP
#SERVER="dyn.local:12323"
SERVER="dyn.local 12323"

# Set the zone to modify, it can be any zone previous key has permissions to modify
ZONE="dyn.commoninternet.net"

# Get your public IP address in the quickest and fanciest
# way to if you have bind-tools installed
#IP=`dig TXT +short o-o.myaddr.l.google.com @ns1.google.com | awk -F'"' '{ print $2}'`
#OLDIP=`dig $HOST +short @8.8.8.8`
IP="1.1.1.9"
OLDIP="0.0.0.0"

if [ "$IP" != "$OLDIP" ];
then
    $ECHO "server $SERVER" > /tmp/nsupdate
    $ECHO "debug yes" >> /tmp/nsupdate
    $ECHO "zone $ZONE" >> /tmp/nsupdate
#    $ECHO "update delete $HOST" >> /tmp/nsupdate
    $ECHO "update add $HOST 600 A $IP" >> /tmp/nsupdate
    $ECHO "send" >> /tmp/nsupdate
else
    $ECHO "No update needed, exiting..."
fi
$NSUPDATE -k ${KEY} -v /tmp/nsupdate