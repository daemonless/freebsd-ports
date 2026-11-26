--- src/commands/setup.rs.orig	2026-11-23 09:24:23 UTC
+++ src/commands/setup.rs
@@ -84,9 +84,14 @@ impl Setup {
                 links
                     .iter()
                     .filter(|l| {
-                        !l.header
+                        // Ignore unconfigured VNET pseudo-interfaces such as pflog0.
+                        l.header
                             .flags
-                            .contains(netlink_packet_route::link::LinkFlags::Loopback)
+                            .contains(netlink_packet_route::link::LinkFlags::Up)
+                            && !l
+                                .header
+                                .flags
+                                .contains(netlink_packet_route::link::LinkFlags::Loopback)
                     })
                     .count()
             })
