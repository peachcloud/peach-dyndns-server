#!/usr/bin/env bash
# this script rebuilds the peach-dyndns-server for prod deployment using the dev folder as the source repo
cd /srv/peachcloud/peach-dyndns-server/dev-peach-dyndns
cargo deb
sudo dpkg -i target/debian/peach-dyndns-server_0.1.0_amd64.deb
sudo systemctl restart peach-dyndns-server
sudo systemctl restart nginx