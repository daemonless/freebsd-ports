--- src/server/serve.rs.orig
+++ src/server/serve.rs
@@ -114,8 +114,10 @@ pub async fn serve(
                     error!("{e}");
                 };
             }
-            // Block until resolv.conf is changed, monitored via inotify. Then reload nameservers
-            _ = event_stream.as_mut().unwrap().next(), if event_stream.is_some() => {
+            // Block until resolv.conf is changed, monitored via inotify. Then reload nameservers.
+            // select! evaluates a disabled branch's future expression before checking the guard,
+            // so avoid unwrapping a missing stream outside the async block.
+            Some(_) = next_inotify_event(&mut event_stream), if event_stream.is_some() => {
                 let upstream_resolvers = match get_upstream_resolvers() {
                     Ok(ns) => ns,
                     Err(err) => {
@@ -398,6 +400,13 @@ fn get_upstream_resolvers() -> AardvarkResult<Vec<SocketAddr>> {
     parse_resolv_conf(&buf)
 }
 
+async fn next_inotify_event<S, T>(stream: &mut Option<S>) -> Option<T>
+where
+    S: futures::Stream<Item = T> + Unpin,
+{
+    stream.as_mut()?.next().await
+}
+
 fn get_inotify_event_stream() -> Option<EventStream<[u8; 1024]>> {
     // Min buffer size is 272 (sizeof(struct inotify_event) + NAME_MAX + 1)
     let buffer = [0; 1024];
@@ -616,4 +625,30 @@ nameserver 1.1.1.2",
         .expect("failed to parse");
         assert_eq!(res, vec![IP_FE80_1]);
     }
+
+    #[test]
+    fn test_select_without_inotify_stream_does_not_panic() {
+        let rt = tokio::runtime::Builder::new_current_thread()
+            .enable_all()
+            .build()
+            .unwrap();
+        rt.block_on(async {
+            let mut none_stream: Option<futures::stream::Empty<()>> = None;
+            let hit = tokio::select! {
+                _ = std::future::ready(()) => "ready",
+                Some(_) = next_inotify_event(&mut none_stream), if none_stream.is_some() => "inotify",
+            };
+            assert_eq!(hit, "ready");
+
+            let mut some_stream = Some(futures::stream::iter(vec![1_u8]));
+            let hit = tokio::select! {
+                _ = std::future::pending::<()>() => "pending",
+                Some(v) = next_inotify_event(&mut some_stream), if some_stream.is_some() => {
+                    assert_eq!(v, 1);
+                    "inotify"
+                }
+            };
+            assert_eq!(hit, "inotify");
+        });
+    }
 }
