--- docs/netavark-pf.7.md.orig	2026-11-23 09:24:23 UTC
+++ docs/netavark-pf.7.md
@@ -27,9 +27,9 @@ The older root-ruleset `table <netavark-subnets> persi
 Without these lines `pfctl` still accepts the rules, but pf never evaluates them: containers have no outbound NAT, no isolation, and published ports do not answer.
 The older root-ruleset `table <netavark-subnets> persist` and `table <netavark-isolated> persist` declarations are no longer needed; keeping them in an existing `pf.conf` is harmless, though Netavark leaves them empty.
 
-Netavark adds no `pass` rules for ordinary container traffic.
+Netavark adds no general `pass` rules for ordinary container traffic.
 A ruleset that blocks by default must pass traffic to and from the container bridges or subnets itself (for example `pass on podman0`, or `pass from 10.88.0.0/16` and `pass to 10.88.0.0/16`).
-`rdr pass` only admits a redirected connection on its way in; it still has to leave through the bridge.
+For published ports, `rdr pass` admits redirected connections on entry with `tag netavark-rdr` and a `pass out quick` rule on the destination bridge creates outbound state so redirected traffic reaches the container and replies pass `<isolated>`.
 
 To reach published ports through the host's own addresses, including `127.0.0.1`, set `net.pf.filter_local` before pf is enabled:
 
@@ -58,13 +58,12 @@ Each container with published ports gets an anchor nam
 Each network gets an anchor named after the network (`netavark/<hash>`).
 It holds the outbound NAT on the interface of the host's default route, and the isolation rules when the network is internal or has the `isolate` option.
 Each container with published ports gets an anchor named after the network and the container ID (`netavark/<hash>-<id>`).
-It holds `rdr pass` rules that redirect the host ports to the container, and a NAT rule on the bridge so that the container itself and the host's loopback can reach the published ports (widened to all sources when the container has additional networks or `no_default_route`, so replies return through the bridge).
+It holds `rdr pass ... tag netavark-rdr` rules that redirect the host ports to the container, a NAT rule on the bridge so that the container itself and the host's loopback can reach the published ports (widened to all sources when the container has additional networks or `no_default_route`, so replies return through the bridge), and a `pass out quick` rule on the bridge matching `tagged netavark-rdr` so outbound bridge state permits replies from isolated networks.
 When DNS is enabled, port 53 on the gateway is redirected to aardvark-dns.
 
 Each isolated network anchor carries its own `table <isolated>` listing the subnets of the peer Netavark networks it may not reach (`isolate=strict` lists every other Netavark subnet; `isolate=true` lists the subnets of other isolated networks).
 Netavark updates the `<isolated>` tables across active network anchors whenever a network is set up or torn down.
-Isolation matches the nftables driver: an isolated network cannot open connections to other Netavark networks (with `isolate=strict`, any of them; otherwise, the isolated ones).
-Incoming traffic is not filtered, and redirected connections are admitted by `rdr pass` before any filter rule.
+Isolation matches the nftables driver: an isolated network cannot open connections to other Netavark networks (with `isolate=strict`, any of them; otherwise, the isolated ones), while published ports remain reachable across networks because `rdr pass` sets `tag netavark-rdr` and the container anchor creates outbound state on the destination bridge.
 Internal networks reach only their gateway.
 
 The anchors can be listed with:
