

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