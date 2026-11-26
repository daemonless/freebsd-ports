--- src/firewall/pf.rs.orig	2026-11-23 09:24:23 UTC
+++ src/firewall/pf.rs
@@ -13,10 +13,11 @@
 //! One anchor per network holds its outbound NAT, its isolation rules, and
 //! when isolated its own `<isolated>` table listing the peer subnets it may
 //! not reach; one anchor per network and container holds that container's
-//! port redirects and the bridge NAT rules for them. Because `<isolated>`
-//! is defined inside the network anchor rather than in the main ruleset,
-//! rules and isolation tables survive `service pf reload`, `service pf restart`,
-//! and `pfctl -F all && pfctl -f /etc/pf.conf`.
+//! port redirects, bridge NAT rules, and outbound state rules for
+//! redirected traffic (`tag netavark-rdr`) so replies pass `<isolated>`.
+//! Because `<isolated>` is defined inside the network anchor rather than in
+//! the main ruleset, rules and isolation tables survive `service pf reload`,
+//! `service pf restart`, and `pfctl -F all && pfctl -f /etc/pf.conf`.
 
 use std::io::Write;
 use std::net::IpAddr;
@@ -40,6 +41,8 @@ const ISOLATED_TABLE: &str = "isolated";
 const ANCHOR_PREFIX: &str = "netavark/";
 /// Anchor-local table listing the peer subnets that this network may not reach.
 const ISOLATED_TABLE: &str = "isolated";
+/// Tag applied to port-forwarded connections so replies pass `<isolated>`.
+const RDR_TAG: &str = "netavark-rdr";
 
 pub struct Pf {
     config_dir: PathBuf,
@@ -275,6 +278,7 @@ fn container_rules(
     bridge: &dyn Fn(&IpNet) -> Option<String>,
 ) -> NetavarkResult<String> {
     let mut rules = String::new();
+    let mut filter_rules = String::new();
 
     if setup_portfw.dns_port != 53 {
         for ip in setup_portfw.dns_server_ips {
@@ -288,22 +292,39 @@ fn container_rules(
 
     if !setup_portfw.internal {
         if let (Some(ip), Some(subnet)) = (setup_portfw.container_ip_v4, setup_portfw.subnet_v4) {
-            port_rules(&mut rules, ip, subnet, setup_portfw, bridge)?;
+            port_rules(
+                &mut rules,
+                &mut filter_rules,
+                ip,
+                subnet,
+                setup_portfw,
+                bridge,
+            )?;
         }
         if let (Some(ip), Some(subnet)) = (setup_portfw.container_ip_v6, setup_portfw.subnet_v6) {
-            port_rules(&mut rules, ip, subnet, setup_portfw, bridge)?;
+            port_rules(
+                &mut rules,
+                &mut filter_rules,
+                ip,
+                subnet,
+                setup_portfw,
+                bridge,
+            )?;
         }
     }
+    rules.push_str(&filter_rules);
     Ok(rules)
 }
 
-/// Redirects for one address family of a container, plus the NAT rule on
-/// the bridge so replies return through it: limited to loopback and the
-/// bridge's own subnet when this is the sole default route, or all
-/// sources when another network or `no_default_route` would send replies
-/// elsewhere.
+/// Redirects and bridge NAT for one address family of a container, plus an
+/// outbound state rule on the bridge for redirected traffic (`tag netavark-rdr`)
+/// so replies from the container pass peer `<isolated>` rules. NAT is limited
+/// to loopback and the bridge's own subnet when this is the sole default route,
+/// or widened to all sources when another network or `no_default_route` would
+/// send replies elsewhere.
 fn port_rules(
     rules: &mut String,
+    filter_rules: &mut String,
     ip: IpAddr,
     subnet: IpNet,
     setup_portfw: &PortForwardConfig,
@@ -318,6 +339,7 @@ fn port_rules(
     } else {
         format!("{{ (lo0), {subnet} }}")
     };
+    let mut emitted = false;
 
     for port in setup_portfw.port_mappings.iter().flatten() {
         let Some(host) = host_address(port, ip.is_ipv4())? else {
@@ -334,8 +356,9 @@ fn port_rules(
             if proto.is_empty() {
                 continue;
             }
+            emitted = true;
             rules.push_str(&format!(
-                "rdr pass {family} proto {proto} from any to {host} port {host_ports} -> {ip} port {container_ports}\n"
+                "rdr pass {family} proto {proto} from any to {host} port {host_ports} tag {RDR_TAG} -> {ip} port {container_ports}\n"
             ));
             rules.push_str(&format!(
                 "nat on {bridge} {family} proto {proto} from {nat_src} to {ip} port {} -> ({bridge})\n",
@@ -343,6 +366,13 @@ fn port_rules(
             ));
         }
     }
+    if emitted {
+        // rdr pass skips filter evaluation only on entry; without an outbound
+        // state on the target bridge, isolated peer bridges drop the reply.
+        filter_rules.push_str(&format!(
+            "pass out quick on {bridge} {family} from any to {ip} tagged {RDR_TAG}\n"
+        ));
+    }
     Ok(())
 }
 
@@ -716,12 +746,13 @@ mod tests {
         assert_eq!(
             rules,
             "rdr pass inet proto { tcp, udp } from any to 10.89.0.1 port 53 -> 10.89.0.1 port 5353\n\
-             rdr pass inet proto tcp from any to (self) port 18080 -> 10.89.0.2 port 8000\n\
+             rdr pass inet proto tcp from any to (self) port 18080 tag netavark-rdr -> 10.89.0.2 port 8000\n\
              nat on nvtest0 inet proto tcp from { (lo0), 10.89.0.0/24 } to 10.89.0.2 port 8000 -> (nvtest0)\n\
-             rdr pass inet proto udp from any to 192.168.86.5 port 2000:2010 -> 10.89.0.2 port 3000:*\n\
+             rdr pass inet proto udp from any to 192.168.86.5 port 2000:2010 tag netavark-rdr -> 10.89.0.2 port 3000:*\n\
              nat on nvtest0 inet proto udp from { (lo0), 10.89.0.0/24 } to 10.89.0.2 port 3000:3010 -> (nvtest0)\n\
-             rdr pass inet proto tcp from any to 192.168.86.5 port 2000:2010 -> 10.89.0.2 port 3000:*\n\
-             nat on nvtest0 inet proto tcp from { (lo0), 10.89.0.0/24 } to 10.89.0.2 port 3000:3010 -> (nvtest0)\n"
+             rdr pass inet proto tcp from any to 192.168.86.5 port 2000:2010 tag netavark-rdr -> 10.89.0.2 port 3000:*\n\
+             nat on nvtest0 inet proto tcp from { (lo0), 10.89.0.0/24 } to 10.89.0.2 port 3000:3010 -> (nvtest0)\n\
+             pass out quick on nvtest0 inet from any to 10.89.0.2 tagged netavark-rdr\n"
         );
         assert_eq!(
             container_anchor("hash", "0123456789abcdef"),
@@ -755,14 +786,45 @@ mod tests {
         let rules = container_rules(&setup, &bridge).unwrap();
         assert_eq!(
             rules,
-            "rdr pass inet proto tcp from any to (self) port 18080 -> 10.89.0.2 port 8000\n\
+            "rdr pass inet proto tcp from any to (self) port 18080 tag netavark-rdr -> 10.89.0.2 port 8000\n\
              nat on nvtest0 inet proto tcp from any to 10.89.0.2 port 8000 -> (nvtest0)\n\
-             rdr pass inet proto udp from any to (self) port 2000:2010 -> 10.89.0.2 port 3000:*\n\
+             rdr pass inet proto udp from any to (self) port 2000:2010 tag netavark-rdr -> 10.89.0.2 port 3000:*\n\
              nat on nvtest0 inet proto udp from any to 10.89.0.2 port 3000:3010 -> (nvtest0)\n\
-             rdr pass inet6 proto tcp from any to (self) port 18080 -> fd00:89::2 port 8000\n\
+             rdr pass inet6 proto tcp from any to (self) port 18080 tag netavark-rdr -> fd00:89::2 port 8000\n\
              nat on nvtest0 inet6 proto tcp from any to fd00:89::2 port 8000 -> (nvtest0)\n\
-             rdr pass inet6 proto udp from any to (self) port 2000:2010 -> fd00:89::2 port 3000:*\n\
-             nat on nvtest0 inet6 proto udp from any to fd00:89::2 port 3000:3010 -> (nvtest0)\n"
+             rdr pass inet6 proto udp from any to (self) port 2000:2010 tag netavark-rdr -> fd00:89::2 port 3000:*\n\
+             nat on nvtest0 inet6 proto udp from any to fd00:89::2 port 3000:3010 -> (nvtest0)\n\
+             pass out quick on nvtest0 inet from any to 10.89.0.2 tagged netavark-rdr\n\
+             pass out quick on nvtest0 inet6 from any to fd00:89::2 tagged netavark-rdr\n"
+        );
+    }
+
+    #[test]
+    fn container_port_rules_dual_stack_v4_only_mapping() {
+        let mappings = Some(vec![mapping("127.0.0.1", 18080, 8000, "tcp", 1)]);
+        let dns = vec![];
+        let setup = PortForwardConfig {
+            container_id: "0123456789abcdef".into(),
+            network_id: "id".into(),
+            port_mappings: &mappings,
+            network_name: "nvtest".into(),
+            network_hash_name: "hash".into(),
+            container_ip_v4: Some("10.89.0.2".parse().unwrap()),
+            subnet_v4: Some("10.89.0.0/24".parse().unwrap()),
+            container_ip_v6: Some("fd00:89::2".parse().unwrap()),
+            subnet_v6: Some("fd00:89::/64".parse().unwrap()),
+            dns_port: 53,
+            dns_server_ips: &dns,
+            internal: false,
+            snat_all: false,
+        };
+        let bridge = |_: &IpNet| Some("nvtest0".to_string());
+        let rules = container_rules(&setup, &bridge).unwrap();
+        assert_eq!(
+            rules,
+            "rdr pass inet proto tcp from any to 127.0.0.1 port 18080 tag netavark-rdr -> 10.89.0.2 port 8000\n\
+             nat on nvtest0 inet proto tcp from { (lo0), 10.89.0.0/24 } to 10.89.0.2 port 8000 -> (nvtest0)\n\
+             pass out quick on nvtest0 inet from any to 10.89.0.2 tagged netavark-rdr\n"
         );
     }
 }
