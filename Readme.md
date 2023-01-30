# Crabby DNS

Crabby DNS is a simple server that aims to replace services like `systemd-resolve` in order to run a local pihole-like DNS-based AD filtering. It is written in Rust and is _blazingly fast_.

The project is still under construction. Right now we have the following features completed:

- [ ] Parsing of DNS Packets (in progress).
- [ ] Creation of a DNS Packet.
- [ ] Transparent DNS proxy service (something that listens to DNS requests and routes them to a third-party server).
- [ ] Installable service that replaces stuff like `systemd-resolve` on the local installation.
- [ ] Blacklisting of AD-associated DNS endpoints.
- [ ] Reporting.
- [ ] Configuration file-based settings like number of worker threads and blocklist URLs (like pihole blocklist servers).