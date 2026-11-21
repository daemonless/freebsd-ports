--- src/network/constants.rs.orig	2026-11-20 22:33:00 UTC
+++ src/network/constants.rs
@@ -38,7 +38,10 @@ pub const NO_CONTAINER_INTERFACE_ERROR: &str = "no con
 pub const NO_CONTAINER_INTERFACE_ERROR: &str = "no container interface name given";
 
 /// make sure this is the same rootful default as used in podman.
+#[cfg(not(target_os = "freebsd"))]
 pub const DEFAULT_CONFIG_DIR: &str = "/run/containers/networks";
+#[cfg(target_os = "freebsd")]
+pub const DEFAULT_CONFIG_DIR: &str = "/var/run/containers/networks";
 
 pub const MAX_INTERFACE_NAME_LEN: usize = 15;
 
