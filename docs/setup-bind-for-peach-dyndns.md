The code in this repository assumes that a bind9 is installed and running on the same server as the peach-dyndns server. 
The configuration of this bind9 server could be automated, but for now it is just done manually,
with documentation of the server configuration here. 


1. Sudoers File
The following goes into `/etc/sudoers.d/bindctl` to enable peach-dyndns to reload bind. 
```
#
# Allow server to reload bind
#

# User alias for bind-ctl which can reload bind
User_Alias  BIND_CTRL = peach-dynds

# Command alias for reboot and shutdown
Cmnd_Alias  RELOADBIND = /bin/reloadbind

# Allow BIND_CTRL users to execute RELOADBIND command without password
BIND_CTRL  ALL=(ALL) NOPASSWD: RELOADBIND
```

2. /bin/reloadbind
/bin/reloadbind is a script with the following content:
```
/bin/systemctl reload bind9
```

3. creation of peach-dyndns user, who also belongs to bind group


4. bind9 configuration
```apt-get install bind9```

peach-dyndns then dynamically configures:
/etc/bind/named.conf.local
/etc/bind/peach-dyndns.keys
/var/lib/bind/*

All the files in /etc/bind and /var/lib/bind should have permissions as root:bind.