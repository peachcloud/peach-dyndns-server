# For each subdomain,
# - generate a new ddns key (tsig-keygen -a hmac-md5 {{subdomain}}.dyn.commoninternet.net) and append it to /etc/bind/dyn.commoninternet.net.keys
# - add a zone section to /etc/bind/named.conf.local, associating the key with the subdomain
# - add a minimal zone file to /var/lib/bind/subdomain.dyn.commoninternet.net
# - reload bind and return the secret key to the client

SUBDOMAIN=$1
BASE_DOMAIN=dyn.commoninternet.net
FULL_DOMAIN="${SUBDOMAIN}.${BASE_DOMAIN}"
echo "[generating zone for ${FULL_DOMAIN}]"

tsig-keygen -a hmac-md5 {{subdomain}}.dyn.commoninternet.net