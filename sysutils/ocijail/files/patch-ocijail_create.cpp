--- ocijail/create.cpp.orig	2024-06-05 00:00:00 UTC
+++ ocijail/create.cpp
@@ -6,6 +6,7 @@
 #include <unistd.h>
 #include <iostream>
 #include <sstream>
+#include <unordered_set>

 #include "ocijail/create.h"
 #include "ocijail/hook.h"
@@ -214,6 +215,7 @@
     // Get the parent jail name and requested vnet type (if any)
     std::optional<std::string> parent_jail;
     auto vnet = jail::INHERIT;
+    std::vector<std::string> allow_params;
     if (config.contains("annotations")) {
         auto config_annotations = config["annotations"];
         if (config_annotations.contains("org.freebsd.parentJail")) {
@@ -232,6 +234,39 @@
                     "bad value for org.freebsd.jail.vnet: " + val);
             }
         }
+
+        // Check for allow.* annotations (e.g., org.freebsd.jail.allow.mlock).
+        // Only parameters known to the FreeBSD jail subsystem are accepted;
+        // unknown parameters are logged and ignored. The kernel remains the
+        // final authority and will reject any parameter it does not recognise.
+        static const std::unordered_set<std::string> known_allow_params = {
+            "allow.adjtime",       "allow.chflags",
+            "allow.extattr",       "allow.mlock",
+            "allow.mount",         "allow.mount.devfs",
+            "allow.mount.fdescfs", "allow.mount.nullfs",
+            "allow.mount.procfs",  "allow.mount.tmpfs",
+            "allow.mount.zfs",     "allow.nfsd",
+            "allow.quotas",        "allow.raw_sockets",
+            "allow.read_msgbuf",   "allow.reserved_ports",
+            "allow.routing",       "allow.set_hostname",
+            "allow.setaudit",      "allow.settime",
+            "allow.socket_af",     "allow.suser",
+            "allow.sysvipc",       "allow.unprivileged_parent_tampering",
+            "allow.unprivileged_proc_debug",
+        };
+        const std::string allow_prefix = "org.freebsd.jail.allow.";
+        for (auto& [key, value] : config_annotations.items()) {
+            if (key.starts_with(allow_prefix) && value.is_string()) {
+                std::string param = "allow." + key.substr(allow_prefix.size());
+                std::string val = value;
+                if (!known_allow_params.count(param)) {
+                    app_.log() << "warning: unknown jail allow annotation '"
+                               << key << "', ignoring";
+                } else if (val == "true" || val == "1") {
+                    allow_params.push_back(param);
+                }
+            }
+        }
     }

     // Create a jail config from the OCI config
@@ -247,6 +282,10 @@
     if (allow_chflags) {
         jconf.set("allow.chflags");
     }
+    // Set additional allow parameters from annotations
+    for (const auto& param : allow_params) {
+        jconf.set(param);
+    }
     if (root_readonly) {
         jconf.set("path", readonly_root_path);
     } else {
