# Run with

```
cargo run --target x86_64-fortanix-unknown-sgx
```

Error log:
```
$ cargo run --target x86_64-fortanix-unknown-sgx
    Updating git repository `https://github.com/fortanix/rust-mbedtls`
    Updating git repository `https://github.com/mzohreva/mio`
    Updating git repository `https://github.com/fortanix/tokio.git`
warning: Patch `mbedtls v0.7.1 (https://github.com/fortanix/rust-mbedtls?branch=raoul/tokio-1.15.0#7c128d03)` was not used in the crate graph.
Patch `mio v0.7.6 (https://github.com/mzohreva/mio?branch=mz/sgx-port-0.7.6#b4370d8b)` was not used in the crate graph.
Patch `tokio v1.33.0 (https://github.com/fortanix/tokio.git?branch=raoul/tokio-1.33.0-sgx#beabb64c)` was not used in the crate graph.
Check that the patched package version and available features are compatible
with the dependency requirements. If the patch has a different version from
what is locked in the Cargo.lock file, run `cargo update` to use the new
version. This may also occur with an optional dependency that is not enabled.
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.69
   Compiling futures-core v0.3.29
   Compiling unicode-ident v1.0.12
   Compiling bytes v1.5.0
   Compiling pin-project-lite v0.2.13
   Compiling version_check v0.9.4
   Compiling typenum v1.17.0
   Compiling futures-task v0.3.29
   Compiling fnv v1.0.7
   Compiling itoa v1.0.9
   Compiling once_cell v1.18.0
   Compiling socket2 v0.5.5
   Compiling mio v0.8.9
error[E0583]: file not found for module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:179:1
    |
179 | mod sys;
    | ^^^^^^^^
    |
    = help: to create the module `sys`, create file "/home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sys.rs" or "/home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sys/mod.rs"
    = note: if there is a `mod sys` elsewhere in the crate already, import it with `use crate::...` instead

error: Socket2 doesn't support the compile target
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:182:1
    |
182 | compile_error!("Socket2 doesn't support the compile target");
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0432]: unresolved imports `crate::sys::c_int`, `crate::sys::sa_family_t`, `crate::sys::sockaddr`, `crate::sys::sockaddr_in`, `crate::sys::sockaddr_in6`, `crate::sys::sockaddr_storage`, `crate::sys::socklen_t`, `crate::sys::AF_INET`, `crate::sys::AF_INET6`, `crate::sys::AF_UNIX`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sockaddr.rs:11:5
   |
11 |     c_int, sa_family_t, sockaddr, sockaddr_in, sockaddr_in6, sockaddr_storage, socklen_t, AF_INET,
   |     ^^^^^  ^^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^  ^^^^^^^ no `AF_INET` in `sys`
   |     |      |            |         |            |             |                 |
   |     |      |            |         |            |             |                 no `socklen_t` in `sys`
   |     |      |            |         |            |             no `sockaddr_storage` in `sys`
   |     |      |            |         |            no `sockaddr_in6` in `sys`
   |     |      |            |         no `sockaddr_in` in `sys`
   |     |      |            no `sockaddr` in `sys`
   |     |      no `sa_family_t` in `sys`
   |     no `c_int` in `sys`
12 |     AF_INET6, AF_UNIX,
   |     ^^^^^^^^  ^^^^^^^ no `AF_UNIX` in `sys`
   |     |
   |     no `AF_INET6` in `sys`
   |
   = help: consider importing one of these items instead:
           std::ffi::c_int
           std::os::raw::c_int
           core::ffi::c_int
   = help: consider importing this module instead:
           crate::sockaddr

error[E0432]: unresolved imports `crate::sys::c_int`, `crate::sys::getsockopt`, `crate::sys::setsockopt`, `crate::sys::Bool`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:23:24
   |
23 | use crate::sys::{self, c_int, getsockopt, setsockopt, Bool};
   |                        ^^^^^  ^^^^^^^^^^  ^^^^^^^^^^  ^^^^ no `Bool` in `sys`
   |                        |      |           |
   |                        |      |           no `setsockopt` in `sys`
   |                        |      no `getsockopt` in `sys`
   |                        no `c_int` in `sys`
   |
   = help: consider importing one of these items instead:
           std::ffi::c_int
           std::os::raw::c_int
           core::ffi::c_int

error[E0432]: unresolved import `sys::c_int`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:184:5
    |
184 | use sys::c_int;
    |     ^^^^^^^^^^ no `c_int` in `sys`
    |
help: consider importing one of these items instead
    |
184 | use core::ffi::c_int;
    |     ~~~~~~~~~~~~~~~~
184 | use std::ffi::c_int;
    |     ~~~~~~~~~~~~~~~
184 | use std::os::raw::c_int;
    |     ~~~~~~~~~~~~~~~~~~~

   Compiling futures-util v0.3.29
error[E0432]: unresolved import `crate::sys::IoSourceState`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:12:5
   |
12 | use crate::sys::IoSourceState;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `IoSourceState` in `sys`

error[E0432]: unresolved import `crate::sys::tcp`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:15:17
   |
15 | use crate::sys::tcp::{bind, listen, new_for_addr};
   |                 ^^^ could not find `tcp` in `sys`

error[E0432]: unresolved import `crate::sys::tcp`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/stream.rs:13:17
   |
13 | use crate::sys::tcp::{connect, new_for_addr};
   |                 ^^^ could not find `tcp` in `sys`

error[E0433]: failed to resolve: could not find `Selector` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/poll.rs:308:18
    |
308 |             sys::Selector::new().map(|selector| Poll {
    |                  ^^^^^^^^ could not find `Selector` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:24:14
   |
24 |         sys::event::token(&self.inner)
   |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:38:14
   |
38 |         sys::event::is_readable(&self.inner)
   |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:43:14
   |
43 |         sys::event::is_writable(&self.inner)
   |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:68:14
   |
68 |         sys::event::is_error(&self.inner)
   |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:99:14
   |
99 |         sys::event::is_read_closed(&self.inner)
   |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:129:14
    |
129 |         sys::event::is_write_closed(&self.inner)
    |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:151:14
    |
151 |         sys::event::is_priority(&self.inner)
    |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:173:14
    |
173 |         sys::event::is_aio(&self.inner)
    |              ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `event` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:183:14
    |
183 |         sys::event::is_lio(&self.inner)
    |              ^^^^^ could not find `event` in `sys`

   Compiling pin-utils v0.1.0
error[E0433]: failed to resolve: could not find `event` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:221:26
    |
221 |                     sys::event::debug_details(f, self.0)
    |                          ^^^^^ could not find `event` in `sys`

error[E0433]: failed to resolve: could not find `tcp` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:103:18
    |
103 |             sys::tcp::accept(inner).map(|(stream, addr)| (TcpStream::from_std(stream), addr))
    |                  ^^^ could not find `tcp` in `sys`

error[E0433]: failed to resolve: could not find `udp` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/udp.rs:122:14
    |
122 |         sys::udp::bind(addr).map(UdpSocket::from_std)
    |              ^^^ could not find `udp` in `sys`

error[E0433]: failed to resolve: could not find `udp` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/udp.rs:544:14
    |
544 |         sys::udp::only_v6(&self.inner)
    |              ^^^ could not find `udp` in `sys`

error[E0412]: cannot find type `Selector` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/poll.rs:259:20
    |
259 |     selector: sys::Selector,
    |                    ^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `Selector` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/poll.rs:702:44
    |
702 |     pub(crate) fn selector(&self) -> &sys::Selector {
    |                                            ^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `Waker` in module `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/waker.rs:79:17
   |
79 |     inner: sys::Waker,
   |                 ^^^^^ not found in `sys`
   |
help: consider importing one of these items
   |
1  + use core::task::Waker;
   |
1  + use crate::Waker;
   |
1  + use std::task::Waker;
   |
help: if you import `Waker`, refer to it directly
   |
79 -     inner: sys::Waker,
79 +     inner: Waker,
   |

error[E0433]: failed to resolve: could not find `Waker` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/waker.rs:87:14
   |
87 |         sys::Waker::new(registry.selector(), token).map(|inner| Waker { inner })
   |              ^^^^^ could not find `Waker` in `sys`
   |
help: consider importing one of these items
   |
1  + use core::task::Waker;
   |
1  + use crate::Waker;
   |
1  + use std::task::Waker;
   |
help: if you import `Waker`, refer to it directly
   |
87 -         sys::Waker::new(registry.selector(), token).map(|inner| Waker { inner })
87 +         Waker::new(registry.selector(), token).map(|inner| Waker { inner })
   |

error[E0412]: cannot find type `Event` in module `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:18:17
   |
18 |     inner: sys::Event,
   |                 ^^^^^ not found in `sys`
   |
help: consider importing this struct through its public re-export
   |
1  + use crate::event::Event;
   |
help: if you import `Event`, refer to it directly
   |
18 -     inner: sys::Event,
18 +     inner: Event,
   |

error[E0412]: cannot find type `Event` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:187:55
    |
187 |     pub(crate) fn from_sys_event_ref(sys_event: &sys::Event) -> &Event {
    |                                                       ^^^^^ not found in `sys`
    |
help: consider importing this struct through its public re-export
    |
1   + use crate::event::Event;
    |
help: if you import `Event`, refer to it directly
    |
187 -     pub(crate) fn from_sys_event_ref(sys_event: &sys::Event) -> &Event {
187 +     pub(crate) fn from_sys_event_ref(sys_event: &Event) -> &Event {
    |

error[E0412]: cannot find type `Event` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:191:41
    |
191 |             &*(sys_event as *const sys::Event as *const Event)
    |                                         ^^^^^ not found in `sys`
    |
help: consider importing this struct through its public re-export
    |
1   + use crate::event::Event;
    |
help: if you import `Event`, refer to it directly
    |
191 -             &*(sys_event as *const sys::Event as *const Event)
191 +             &*(sys_event as *const Event as *const Event)
    |

error[E0412]: cannot find type `Event` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/event.rs:217:46
    |
217 |             struct EventDetails<'a>(&'a sys::Event);
    |                                              ^^^^^ not found in `sys`
    |
help: consider importing this struct through its public re-export
    |
1   + use crate::event::Event;
    |
help: if you import `Event`, refer to it directly
    |
217 -             struct EventDetails<'a>(&'a sys::Event);
217 +             struct EventDetails<'a>(&'a Event);
    |

error[E0412]: cannot find type `Events` in module `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/events.rs:43:17
   |
43 |     inner: sys::Events,
   |                 ^^^^^^ not found in `sys`
   |
help: consider importing this struct through its public re-export
   |
1  + use crate::Events;
   |
help: if you import `Events`, refer to it directly
   |
43 -     inner: sys::Events,
43 +     inner: Events,
   |

error[E0433]: failed to resolve: could not find `Events` in `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/events.rs:94:25
   |
94 |             inner: sys::Events::with_capacity(capacity),
   |                         ^^^^^^ could not find `Events` in `sys`
   |
help: consider importing this struct through its public re-export
   |
1  + use crate::Events;
   |
help: if you import `Events`, refer to it directly
   |
94 -             inner: sys::Events::with_capacity(capacity),
94 +             inner: Events::with_capacity(capacity),
   |

error[E0412]: cannot find type `Events` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/events.rs:189:47
    |
189 |     pub(crate) fn sys(&mut self) -> &mut sys::Events {
    |                                               ^^^^^^ not found in `sys`
    |
help: consider importing this struct through its public re-export
    |
1   + use crate::Events;
    |
help: if you import `Events`, refer to it directly
    |
189 -     pub(crate) fn sys(&mut self) -> &mut sys::Events {
189 +     pub(crate) fn sys(&mut self) -> &mut Events {
    |

error[E0425]: cannot find value `listener` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:74:24
   |
74 |         set_reuseaddr(&listener.inner, true)?;
   |                        ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `listener` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:76:15
   |
76 |         bind(&listener.inner, addr)?;
   |               ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `listener` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:77:17
   |
77 |         listen(&listener.inner, 1024)?;
   |                 ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `listener` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:78:12
   |
78 |         Ok(listener)
   |            ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `stream` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/stream.rs:90:18
   |
90 |         connect(&stream.inner, addr)?;
   |                  ^^^^^^ not found in this scope

error[E0425]: cannot find value `stream` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/stream.rs:91:12
   |
91 |         Ok(stream)
   |            ^^^^^^ not found in this scope

error[E0433]: failed to resolve: could not find `MaybeUninitSlice` in `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:405:31
    |
405 |         MaybeUninitSlice(sys::MaybeUninitSlice::new(buf))
    |                               ^^^^^^^^^^^^^^^^ could not find `MaybeUninitSlice` in `sys`

   Compiling tracing-core v0.1.32
error[E0425]: cannot find function `unix_sockaddr` in module `crate::sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sockaddr.rs:153:21
    |
153 |         crate::sys::unix_sockaddr(path.as_ref())
    |                     ^^^^^^^^^^^^^ not found in `crate::sys`

error[E0425]: cannot find function `from_in_addr` in module `crate::sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sockaddr.rs:215:34
    |
215 |             let ip = crate::sys::from_in_addr(addr.sin_addr);
    |                                  ^^^^^^^^^^^^ not found in `crate::sys`

error[E0425]: cannot find function `from_in6_addr` in module `crate::sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sockaddr.rs:222:34
    |
222 |             let ip = crate::sys::from_in6_addr(addr.sin6_addr);
    |                                  ^^^^^^^^^^^^^ not found in `crate::sys`

error[E0425]: cannot find function `to_in_addr` in module `crate::sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sockaddr.rs:284:44
    |
284 |             storage.sin_addr = crate::sys::to_in_addr(addr.ip());
    |                                            ^^^^^^^^^^ not found in `crate::sys`

error[E0425]: cannot find function `to_in6_addr` in module `crate::sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/sockaddr.rs:317:45
    |
317 |             storage.sin6_addr = crate::sys::to_in6_addr(addr.ip());
    |                                             ^^^^^^^^^^^ not found in `crate::sys`

error[E0412]: cannot find type `Socket` in module `sys`
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:89:38
   |
89 |     pub(crate) fn from_raw(raw: sys::Socket) -> Socket {
   |                                      ^^^^^^ not found in `sys`
   |
help: consider importing this struct through its public re-export
   |
9  + use crate::Socket;
   |
help: if you import `Socket`, refer to it directly
   |
89 -     pub(crate) fn from_raw(raw: sys::Socket) -> Socket {
89 +     pub(crate) fn from_raw(raw: Socket) -> Socket {
   |

error[E0425]: cannot find function `socket_from_raw` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:107:22
    |
107 |                 sys::socket_from_raw(raw)
    |                      ^^^^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `Socket` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:112:41
    |
112 |     pub(crate) fn as_raw(&self) -> sys::Socket {
    |                                         ^^^^^^ not found in `sys`
    |
help: consider importing this struct through its public re-export
    |
9   + use crate::Socket;
    |
help: if you import `Socket`, refer to it directly
    |
112 -     pub(crate) fn as_raw(&self) -> sys::Socket {
112 +     pub(crate) fn as_raw(&self) -> Socket {
    |

error[E0425]: cannot find function `socket_as_raw` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:113:14
    |
113 |         sys::socket_as_raw(&self.inner)
    |              ^^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `Socket` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:116:42
    |
116 |     pub(crate) fn into_raw(self) -> sys::Socket {
    |                                          ^^^^^^ not found in `sys`
    |
help: consider importing this struct through its public re-export
    |
9   + use crate::Socket;
    |
help: if you import `Socket`, refer to it directly
    |
116 -     pub(crate) fn into_raw(self) -> sys::Socket {
116 +     pub(crate) fn into_raw(self) -> Socket {
    |

error[E0425]: cannot find function `socket_into_raw` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:117:14
    |
117 |         sys::socket_into_raw(self.inner)
    |              ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `socket` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:142:14
    |
142 |         sys::socket(domain.0, ty.0, protocol).map(Socket::from_raw)
    |              ^^^^^^ not found in `sys`

error[E0425]: cannot find function `bind` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:187:14
    |
187 |         sys::bind(self.as_raw(), address)
    |              ^^^^ not found in `sys`

error[E0425]: cannot find function `connect` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:206:14
    |
206 |         sys::connect(self.as_raw(), address)
    |              ^^^^^^^ not found in `sys`

error[E0425]: cannot find function `poll_connect` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:241:14
    |
241 |         sys::poll_connect(self, timeout)
    |              ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `listen` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:254:14
    |
254 |         sys::listen(self.as_raw(), backlog)
    |              ^^^^^^ not found in `sys`

error[E0425]: cannot find function `accept` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:306:14
    |
306 |         sys::accept(self.as_raw()).map(|(inner, addr)| (Socket::from_raw(inner), addr))
    |              ^^^^^^ not found in `sys`

error[E0425]: cannot find function `getsockname` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:322:14
    |
322 |         sys::getsockname(self.as_raw())
    |              ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `getpeername` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:337:14
    |
337 |         sys::getpeername(self.as_raw())
    |              ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:343:58
    |
343 |         unsafe { getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_TYPE).map(Type) }
    |                                                          ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_TYPE` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:343:75
    |
343 |         unsafe { getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_TYPE).map(Type) }
    |                                                                           ^^^^^^^ not found in `sys`

error[E0425]: cannot find function `try_clone` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:360:14
    |
360 |         sys::try_clone(self.as_raw()).map(Socket::from_raw)
    |              ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_nonblocking` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:386:14
    |
386 |         sys::set_nonblocking(self.as_raw(), nonblocking)
    |              ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `shutdown` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:395:14
    |
395 |         sys::shutdown(self.as_raw(), how)
    |              ^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `MSG_OOB` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:432:40
    |
432 |         self.recv_with_flags(buf, sys::MSG_OOB)
    |                                        ^^^^^^^ not found in `sys`

error[E0412]: cannot find type `c_int` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:442:21
    |
442 |         flags: sys::c_int,
    |                     ^^^^^ not found in `sys`
    |
help: consider importing one of these items
    |
9   + use core::ffi::c_int;
    |
9   + use std::ffi::c_int;
    |
9   + use std::os::raw::c_int;
    |
help: if you import `c_int`, refer to it directly
    |
442 -         flags: sys::c_int,
442 +         flags: c_int,
    |

error[E0425]: cannot find function `recv` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:444:14
    |
444 |         sys::recv(self.as_raw(), buf, flags)
    |              ^^^^ not found in `sys`

error[E0425]: cannot find function `recv_vectored` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:500:14
    |
500 |         sys::recv_vectored(self.as_raw(), bufs, flags)
    |              ^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `MSG_PEEK` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:517:40
    |
517 |         self.recv_with_flags(buf, sys::MSG_PEEK)
    |                                        ^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `recv_from` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:543:14
    |
543 |         sys::recv_from(self.as_raw(), buf, flags)
    |              ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `recv_from_vectored` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:586:14
    |
586 |         sys::recv_from_vectored(self.as_raw(), bufs, flags)
    |              ^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `MSG_PEEK` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:619:45
    |
619 |         self.recv_from_with_flags(buf, sys::MSG_PEEK)
    |                                             ^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `peek_sender` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:629:14
    |
629 |         sys::peek_sender(self.as_raw())
    |              ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `send` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:661:14
    |
661 |         sys::send(self.as_raw(), buf, flags)
    |              ^^^^ not found in `sys`
    |
help: consider importing one of these items
    |
9   + use std::os::fortanix_sgx::usercalls::raw::UsercallNrs::send;
    |
9   + use std::os::fortanix_sgx::usercalls::raw::send;
    |
9   + use std::os::fortanix_sgx::usercalls::send;
    |
help: if you import `send`, refer to it directly
    |
661 -         sys::send(self.as_raw(), buf, flags)
661 +         send(self.as_raw(), buf, flags)
    |

error[E0425]: cannot find function `send_vectored` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:683:14
    |
683 |         sys::send_vectored(self.as_raw(), bufs, flags)
    |              ^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `MSG_OOB` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:695:40
    |
695 |         self.send_with_flags(buf, sys::MSG_OOB)
    |                                        ^^^^^^^ not found in `sys`

error[E0425]: cannot find function `send_to` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:717:14
    |
717 |         sys::send_to(self.as_raw(), buf, addr, flags)
    |              ^^^^^^^ not found in `sys`

error[E0425]: cannot find function `send_to_vectored` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:741:14
    |
741 |         sys::send_to_vectored(self.as_raw(), bufs, addr, flags)
    |              ^^^^^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `c_int` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:748:65
    |
748 |     pub fn sendmsg(&self, msg: &MsgHdr<'_, '_, '_>, flags: sys::c_int) -> io::Result<usize> {
    |                                                                 ^^^^^ not found in `sys`
    |
help: consider importing one of these items
    |
9   + use core::ffi::c_int;
    |
9   + use std::ffi::c_int;
    |
9   + use std::os::raw::c_int;
    |
help: if you import `c_int`, refer to it directly
    |
748 -     pub fn sendmsg(&self, msg: &MsgHdr<'_, '_, '_>, flags: sys::c_int) -> io::Result<usize> {
748 +     pub fn sendmsg(&self, msg: &MsgHdr<'_, '_, '_>, flags: c_int) -> io::Result<usize> {
    |

error[E0425]: cannot find function `sendmsg` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:749:14
    |
749 |         sys::sendmsg(self.as_raw(), msg, flags)
    |              ^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:843:53
    |
843 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_BROADCAST)
    |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_BROADCAST` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:843:70
    |
843 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_BROADCAST)
    |                                                                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:856:22
    |
856 |                 sys::SOL_SOCKET,
    |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_BROADCAST` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:857:22
    |
857 |                 sys::SO_BROADCAST,
    |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:869:64
    |
869 |         match unsafe { getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_ERROR) } {
    |                                                                ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_ERROR` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:869:81
    |
869 |         match unsafe { getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_ERROR) } {
    |                                                                                 ^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:883:52
    |
883 |             getsockopt::<Bool>(self.as_raw(), sys::SOL_SOCKET, sys::SO_KEEPALIVE)
    |                                                    ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_KEEPALIVE` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:883:69
    |
883 |             getsockopt::<Bool>(self.as_raw(), sys::SOL_SOCKET, sys::SO_KEEPALIVE)
    |                                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:895:22
    |
895 |                 sys::SOL_SOCKET,
    |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_KEEPALIVE` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:896:22
    |
896 |                 sys::SO_KEEPALIVE,
    |                      ^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `linger` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:909:31
    |
909 |             getsockopt::<sys::linger>(self.as_raw(), sys::SOL_SOCKET, sys::SO_LINGER)
    |                               ^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:909:59
    |
909 |             getsockopt::<sys::linger>(self.as_raw(), sys::SOL_SOCKET, sys::SO_LINGER)
    |                                                           ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_LINGER` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:909:76
    |
909 |             getsockopt::<sys::linger>(self.as_raw(), sys::SOL_SOCKET, sys::SO_LINGER)
    |                                                                            ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:930:49
    |
930 |         unsafe { setsockopt(self.as_raw(), sys::SOL_SOCKET, sys::SO_LINGER, linger) }
    |                                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_LINGER` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:930:66
    |
930 |         unsafe { setsockopt(self.as_raw(), sys::SOL_SOCKET, sys::SO_LINGER, linger) }
    |                                                                  ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:942:53
    |
942 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_OOBINLINE)
    |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_OOBINLINE` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:942:70
    |
942 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_OOBINLINE)
    |                                                                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:959:22
    |
959 |                 sys::SOL_SOCKET,
    |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_OOBINLINE` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:960:22
    |
960 |                 sys::SO_OOBINLINE,
    |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:973:53
    |
973 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVBUF)
    |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_RCVBUF` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:973:70
    |
973 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVBUF)
    |                                                                      ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:986:22
    |
986 |                 sys::SOL_SOCKET,
    |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_RCVBUF` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:987:22
    |
987 |                 sys::SO_RCVBUF,
    |                      ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `timeout_opt` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:998:14
    |
998 |         sys::timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVTIMEO)
    |              ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:998:46
    |
998 |         sys::timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVTIMEO)
    |                                              ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_RCVTIMEO` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:998:63
    |
998 |         sys::timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVTIMEO)
    |                                                               ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_timeout_opt` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1006:14
     |
1006 |         sys::set_timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVTIMEO, duration)
     |              ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1006:50
     |
1006 |         sys::set_timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVTIMEO, duration)
     |                                                  ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_RCVTIMEO` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1006:67
     |
1006 |         sys::set_timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_RCVTIMEO, duration)
     |                                                                   ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1016:53
     |
1016 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_REUSEADDR)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_REUSEADDR` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1016:70
     |
1016 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_REUSEADDR)
     |                                                                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1030:22
     |
1030 |                 sys::SOL_SOCKET,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_REUSEADDR` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1031:22
     |
1031 |                 sys::SO_REUSEADDR,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1044:53
     |
1044 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDBUF)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_SNDBUF` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1044:70
     |
1044 |             getsockopt::<c_int>(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDBUF)
     |                                                                      ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1057:22
     |
1057 |                 sys::SOL_SOCKET,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_SNDBUF` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1058:22
     |
1058 |                 sys::SO_SNDBUF,
     |                      ^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `timeout_opt` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1069:14
     |
1069 |         sys::timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDTIMEO)
     |              ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1069:46
     |
1069 |         sys::timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDTIMEO)
     |                                              ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_SNDTIMEO` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1069:63
     |
1069 |         sys::timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDTIMEO)
     |                                                               ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_timeout_opt` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1077:14
     |
1077 |         sys::set_timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDTIMEO, duration)
     |              ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOL_SOCKET` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1077:50
     |
1077 |         sys::set_timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDTIMEO, duration)
     |                                                  ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SO_SNDTIMEO` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1077:67
     |
1077 |         sys::set_timeout_opt(self.as_raw(), sys::SOL_SOCKET, sys::SO_SNDTIMEO, duration)
     |                                                                   ^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `linger` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1081:35
     |
1081 | const fn from_linger(linger: sys::linger) -> Option<Duration> {
     |                                   ^^^^^^ not found in `sys`

error[E0412]: cannot find type `linger` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1089:58
     |
1089 | const fn into_linger(duration: Option<Duration>) -> sys::linger {
     |                                                          ^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `linger` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1091:32
     |
1091 |         Some(duration) => sys::linger {
     |                                ^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `linger` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1095:22
     |
1095 |         None => sys::linger {
     |                      ^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1120:53
     |
1120 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_HDRINCL)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_HDRINCL` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1120:70
     |
1120 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_HDRINCL)
     |                                                                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1149:22
     |
1149 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_HDRINCL` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1150:22
     |
1150 |                 sys::IP_HDRINCL,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `IpMreq` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1206:25
     |
1206 |         let mreq = sys::IpMreq {
     |                         ^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_reuseaddr` in this scope
  --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:74:9
   |
74 |         set_reuseaddr(&listener.inner, true)?;
   |         ^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1207:33
     |
1207 |             imr_multiaddr: sys::to_in_addr(multiaddr),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1208:33
     |
1208 |             imr_interface: sys::to_in_addr(interface),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1210:49
     |
1210 |         unsafe { setsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_ADD_MEMBERSHIP, mreq) }
     |                                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_ADD_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1210:66
     |
1210 |         unsafe { setsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_ADD_MEMBERSHIP, mreq) }
     |                                                                  ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `IpMreq` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1219:25
     |
1219 |         let mreq = sys::IpMreq {
     |                         ^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1220:33
     |
1220 |             imr_multiaddr: sys::to_in_addr(multiaddr),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1221:33
     |
1221 |             imr_interface: sys::to_in_addr(interface),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1226:22
     |
1226 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_DROP_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1227:22
     |
1227 |                 sys::IP_DROP_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_mreqn` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1256:26
     |
1256 |         let mreqn = sys::to_mreqn(multiaddr, interface);
     |                          ^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1260:22
     |
1260 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_ADD_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1261:22
     |
1261 |                 sys::IP_ADD_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_mreqn` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1289:26
     |
1289 |         let mreqn = sys::to_mreqn(multiaddr, interface);
     |                          ^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1293:22
     |
1293 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_DROP_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1294:22
     |
1294 |                 sys::IP_DROP_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `IpMreqSource` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1324:26
     |
1324 |         let mreqs = sys::IpMreqSource {
     |                          ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1325:33
     |
1325 |             imr_multiaddr: sys::to_in_addr(group),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1326:33
     |
1326 |             imr_interface: sys::to_in_addr(interface),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1327:34
     |
1327 |             imr_sourceaddr: sys::to_in_addr(source),
     |                                  ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1332:22
     |
1332 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_ADD_SOURCE_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1333:22
     |
1333 |                 sys::IP_ADD_SOURCE_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `IpMreqSource` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1361:26
     |
1361 |         let mreqs = sys::IpMreqSource {
     |                          ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1362:33
     |
1362 |             imr_multiaddr: sys::to_in_addr(group),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1363:33
     |
1363 |             imr_interface: sys::to_in_addr(interface),
     |                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1364:34
     |
1364 |             imr_sourceaddr: sys::to_in_addr(source),
     |                                  ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1369:22
     |
1369 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_DROP_SOURCE_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1370:22
     |
1370 |                 sys::IP_DROP_SOURCE_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1383:44
     |
1383 |             getsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_IF).map(sys::from_in_addr)
     |                                            ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_MULTICAST_IF` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1383:61
     |
1383 |             getsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_IF).map(sys::from_in_addr)
     |                                                             ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `from_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1383:87
     |
1383 |             getsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_IF).map(sys::from_in_addr)
     |                                                                                       ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1391:30
     |
1391 |         let interface = sys::to_in_addr(interface);
     |                              ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1395:22
     |
1395 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_MULTICAST_IF` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1396:22
     |
1396 |                 sys::IP_MULTICAST_IF,
     |                      ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1409:53
     |
1409 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_LOOP)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_MULTICAST_LOOP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1409:70
     |
1409 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_LOOP)
     |                                                                      ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1422:22
     |
1422 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_MULTICAST_LOOP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1423:22
     |
1423 |                 sys::IP_MULTICAST_LOOP,
     |                      ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1436:53
     |
1436 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_TTL)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_MULTICAST_TTL` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1436:70
     |
1436 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_MULTICAST_TTL)
     |                                                                      ^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1452:22
     |
1452 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_MULTICAST_TTL` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1453:22
     |
1453 |                 sys::IP_MULTICAST_TTL,
     |                      ^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1466:53
     |
1466 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_TTL).map(|ttl| ttl as u32)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_TTL` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1466:70
     |
1466 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_TTL).map(|ttl| ttl as u32)
     |                                                                      ^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1475:49
     |
1475 |         unsafe { setsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_TTL, ttl as c_int) }
     |                                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_TTL` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1475:66
     |
1475 |         unsafe { setsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_TTL, ttl as c_int) }
     |                                                                  ^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1492:49
     |
1492 |         unsafe { setsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_TOS, tos as c_int) }
     |                                                 ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_TOS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1492:66
     |
1492 |         unsafe { setsockopt(self.as_raw(), sys::IPPROTO_IP, sys::IP_TOS, tos as c_int) }
     |                                                                  ^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1511:53
     |
1511 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_TOS).map(|tos| tos as u32)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_TOS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1511:70
     |
1511 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_TOS).map(|tos| tos as u32)
     |                                                                      ^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1538:22
     |
1538 |                 sys::IPPROTO_IP,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_RECVTOS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1539:22
     |
1539 |                 sys::IP_RECVTOS,
     |                      ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1566:53
     |
1566 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_RECVTOS)
     |                                                     ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IP_RECVTOS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1566:70
     |
1566 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IP, sys::IP_RECVTOS)
     |                                                                      ^^^^^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `Ipv6Mreq` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1587:25
     |
1587 |         let mreq = sys::Ipv6Mreq {
     |                         ^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in6_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1588:36
     |
1588 |             ipv6mr_multiaddr: sys::to_in6_addr(multiaddr),
     |                                    ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1595:22
     |
1595 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_ADD_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1596:22
     |
1596 |                 sys::IPV6_ADD_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0422]: cannot find struct, variant or union type `Ipv6Mreq` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1611:25
     |
1611 |         let mreq = sys::Ipv6Mreq {
     |                         ^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `to_in6_addr` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1612:36
     |
1612 |             ipv6mr_multiaddr: sys::to_in6_addr(multiaddr),
     |                                    ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1619:22
     |
1619 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_DROP_MEMBERSHIP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1620:22
     |
1620 |                 sys::IPV6_DROP_MEMBERSHIP,
     |                      ^^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1633:53
     |
1633 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_MULTICAST_HOPS)
     |                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_MULTICAST_HOPS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1633:72
     |
1633 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_MULTICAST_HOPS)
     |                                                                        ^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1647:22
     |
1647 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_MULTICAST_HOPS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1648:22
     |
1648 |                 sys::IPV6_MULTICAST_HOPS,
     |                      ^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1661:53
     |
1661 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_MULTICAST_IF)
     |                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_MULTICAST_IF` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1661:72
     |
1661 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_MULTICAST_IF)
     |                                                                        ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1675:22
     |
1675 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_MULTICAST_IF` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1676:22
     |
1676 |                 sys::IPV6_MULTICAST_IF,
     |                      ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1689:53
     |
1689 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_MULTICAST_LOOP)
     |                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_MULTICAST_LOOP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1689:72
     |
1689 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_MULTICAST_LOOP)
     |                                                                        ^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1702:22
     |
1702 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_MULTICAST_LOOP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1703:22
     |
1703 |                 sys::IPV6_MULTICAST_LOOP,
     |                      ^^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1714:53
     |
1714 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_UNICAST_HOPS)
     |                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_UNICAST_HOPS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1714:72
     |
1714 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_UNICAST_HOPS)
     |                                                                        ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1726:22
     |
1726 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_UNICAST_HOPS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1727:22
     |
1727 |                 sys::IPV6_UNICAST_HOPS,
     |                      ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1740:53
     |
1740 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_V6ONLY)
     |                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_V6ONLY` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1740:72
     |
1740 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_V6ONLY)
     |                                                                        ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1757:22
     |
1757 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_V6ONLY` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1758:22
     |
1758 |                 sys::IPV6_V6ONLY,
     |                      ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1783:53
     |
1783 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_RECVTCLASS)
     |                                                     ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPV6_RECVTCLASS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1783:72
     |
1783 |             getsockopt::<c_int>(self.as_raw(), sys::IPPROTO_IPV6, sys::IPV6_RECVTCLASS)
     |                                                                        ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_IPV6` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1809:22
     |
1809 |                 sys::IPPROTO_IPV6,
     |                      ^^^^^^^^^^^^ not found in `sys`

error[E0599]: no method named `register` found for struct `IoSource<std::net::TcpListener>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:146:20
    |
146 |         self.inner.register(registry, token, interests)
    |                    ^^^^^^^^ method not found in `IoSource<TcpListener>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `register` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `register`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

error[E0425]: cannot find value `IPV6_RECVTCLASS` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1810:22
     |
1810 |                 sys::IPV6_RECVTCLASS,
     |                      ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `keepalive_time` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1849:14
     |
1849 |         sys::keepalive_time(self.as_raw())
     |              ^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_tcp_keepalive` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1986:14
     |
1986 |         sys::set_tcp_keepalive(self.as_raw(), params)
     |              ^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_TCP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1996:52
     |
1996 |             getsockopt::<Bool>(self.as_raw(), sys::IPPROTO_TCP, sys::TCP_NODELAY)
     |                                                    ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `TCP_NODELAY` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:1996:70
     |
1996 |             getsockopt::<Bool>(self.as_raw(), sys::IPPROTO_TCP, sys::TCP_NODELAY)
     |                                                                      ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_TCP` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:2012:22
     |
2012 |                 sys::IPPROTO_TCP,
     |                      ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `TCP_NODELAY` in module `sys`
    --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/socket.rs:2013:22
     |
2013 |                 sys::TCP_NODELAY,
     |                      ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `AF_INET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:213:42
    |
213 |     pub const IPV4: Domain = Domain(sys::AF_INET);
    |                                          ^^^^^^^ not found in `sys`

error[E0425]: cannot find value `AF_INET6` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:216:42
    |
216 |     pub const IPV6: Domain = Domain(sys::AF_INET6);
    |                                          ^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `AF_UNIX` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:219:42
    |
219 |     pub const UNIX: Domain = Domain(sys::AF_UNIX);
    |                                          ^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOCK_STREAM` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:258:40
    |
258 |     pub const STREAM: Type = Type(sys::SOCK_STREAM);
    |                                        ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOCK_DGRAM` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:263:39
    |
263 |     pub const DGRAM: Type = Type(sys::SOCK_DGRAM);
    |                                       ^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOCK_SEQPACKET` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:275:43
    |
275 |     pub const SEQPACKET: Type = Type(sys::SOCK_SEQPACKET);
    |                                           ^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `SOCK_RAW` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:283:37
    |
283 |     pub const RAW: Type = Type(sys::SOCK_RAW);
    |                                     ^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_ICMP` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:310:48
    |
310 |     pub const ICMPV4: Protocol = Protocol(sys::IPPROTO_ICMP);
    |                                                ^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_ICMPV6` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:313:48
    |
313 |     pub const ICMPV6: Protocol = Protocol(sys::IPPROTO_ICMPV6);
    |                                                ^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_TCP` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:316:45
    |
316 |     pub const TCP: Protocol = Protocol(sys::IPPROTO_TCP);
    |                                             ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `IPPROTO_UDP` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:319:45
    |
319 |     pub const UDP: Protocol = Protocol(sys::IPPROTO_UDP);
    |                                             ^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find value `MSG_TRUNC` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:382:23
    |
382 |         self.0 & sys::MSG_TRUNC != 0
    |                       ^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `MaybeUninitSlice` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:390:38
    |
390 | pub struct MaybeUninitSlice<'a>(sys::MaybeUninitSlice<'a>);
    |                                      ^^^^^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `msghdr` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:594:17
    |
594 |     inner: sys::msghdr,
    |                 ^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_msghdr_name` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:616:14
    |
616 |         sys::set_msghdr_name(&mut self.inner, addr);
    |              ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_msghdr_iov` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:626:14
    |
626 |         sys::set_msghdr_iov(&mut self.inner, ptr, bufs.len());
    |              ^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_msghdr_control` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:636:14
    |
636 |         sys::set_msghdr_control(&mut self.inner, ptr, buf.len());
    |              ^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `c_int` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:643:45
    |
643 |     pub fn with_flags(mut self, flags: sys::c_int) -> Self {
    |                                             ^^^^^ not found in `sys`
    |
help: consider importing one of these items
    |
61  + use core::ffi::c_int;
    |
61  + use std::ffi::c_int;
    |
61  + use std::os::raw::c_int;
    |
help: if you import `c_int`, refer to it directly
    |
643 -     pub fn with_flags(mut self, flags: sys::c_int) -> Self {
643 +     pub fn with_flags(mut self, flags: c_int) -> Self {
    |

error[E0425]: cannot find function `set_msghdr_flags` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:644:14
    |
644 |         sys::set_msghdr_flags(&mut self.inner, flags);
    |              ^^^^^^^^^^^^^^^^ not found in `sys`

error[E0412]: cannot find type `msghdr` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:662:17
    |
662 |     inner: sys::msghdr,
    |                 ^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_msghdr_name` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:689:14
    |
689 |         sys::set_msghdr_name(&mut self.inner, addr);
    |              ^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_msghdr_iov` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:698:14
    |
698 |         sys::set_msghdr_iov(&mut self.inner, bufs.as_mut_ptr().cast(), bufs.len());
    |              ^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `set_msghdr_control` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:707:14
    |
707 |         sys::set_msghdr_control(&mut self.inner, buf.as_mut_ptr().cast(), buf.len());
    |              ^^^^^^^^^^^^^^^^^^ not found in `sys`

error[E0425]: cannot find function `msghdr_flags` in module `sys`
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.5/src/lib.rs:713:14
    |
713 |         sys::msghdr_flags(&self.inner)
    |              ^^^^^^^^^^^^ not found in `sys`

error[E0599]: no method named `reregister` found for struct `IoSource<std::net::TcpListener>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:155:20
    |
155 |         self.inner.reregister(registry, token, interests)
    |                    ^^^^^^^^^^ method not found in `IoSource<TcpListener>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `reregister` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `reregister`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

   Compiling futures-sink v0.3.29
error[E0599]: no method named `deregister` found for struct `IoSource<std::net::TcpListener>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/listener.rs:159:20
    |
159 |         self.inner.deregister(registry)
    |                    ^^^^^^^^^^ method not found in `IoSource<TcpListener>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `deregister` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `deregister`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

error[E0599]: no method named `register` found for struct `IoSource<std::net::TcpStream>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/stream.rs:325:20
    |
325 |         self.inner.register(registry, token, interests)
    |                    ^^^^^^^^ method not found in `IoSource<TcpStream>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `register` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `register`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

error[E0599]: no method named `reregister` found for struct `IoSource<std::net::TcpStream>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/stream.rs:334:20
    |
334 |         self.inner.reregister(registry, token, interests)
    |                    ^^^^^^^^^^ method not found in `IoSource<TcpStream>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `reregister` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `reregister`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

error[E0599]: no method named `deregister` found for struct `IoSource<std::net::TcpStream>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/tcp/stream.rs:338:20
    |
338 |         self.inner.deregister(registry)
    |                    ^^^^^^^^^^ method not found in `IoSource<TcpStream>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `deregister` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `deregister`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0412, E0422, E0425, E0432, E0433, E0583.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `socket2` (lib) due to 213 previous errors
warning: build failed, waiting for other jobs to finish...
error[E0599]: no method named `register` found for struct `IoSource<std::net::UdpSocket>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/udp.rs:622:20
    |
622 |         self.inner.register(registry, token, interests)
    |                    ^^^^^^^^ method not found in `IoSource<UdpSocket>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `register` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `register`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

error[E0599]: no method named `reregister` found for struct `IoSource<std::net::UdpSocket>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/udp.rs:631:20
    |
631 |         self.inner.reregister(registry, token, interests)
    |                    ^^^^^^^^^^ method not found in `IoSource<UdpSocket>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `reregister` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `reregister`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

error[E0599]: no method named `deregister` found for struct `IoSource<std::net::UdpSocket>` in the current scope
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/net/udp.rs:635:20
    |
635 |         self.inner.deregister(registry)
    |                    ^^^^^^^^^^ method not found in `IoSource<UdpSocket>`
    |
   ::: /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/io_source.rs:62:1
    |
62  | pub struct IoSource<T> {
    | ---------------------- method `deregister` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `Source` defines an item `deregister`, perhaps you need to implement it
   --> /home/ubuntu/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.9/src/event/source.rs:75:1
    |
75  | pub trait Source {
    | ^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0412, E0425, E0432, E0433, E0599.
error: could not compile `mio` (lib) due to 44 previous errors
```
