use std::{cell::RefCell, rc::Rc};

use sauron_core::{
    dom::{self, DomNode},
    prelude::Node,
    vdom,
};
use sauron_html_parser::{parse_html, raw_html};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Example");

    let ev_callback = |_| {};

    let body_node = DomNode::from(web_sys::Node::from(dom::util::body()));
    let mount_node = Rc::new(RefCell::new(Some(body_node)));

    let new_html = r#"<div>
    
    
 <p>
 [[!meta date="2024-07-19 14:33"]] [[!tag nix nixos libnix fixPath]]
 [[!series libnix]] [[!summary libnix roadmap]]
</p>
<p>
 [[!img media/nlnet-logo.gif class="noFancy" style="float: right"]] [[!img
 posts/libnix/Nix_snowflake_windows.svg class="noFancy" style="float:
 right" width="200px"]]
</p>
<h1 id="motivation121111s1111111111111123412112111132">
 motivation121111s1111111111111123412112111132
</h1>
<p>
 status of <strong>native windows nix using MinGW</strong> from my series
 <a href="https://lastlog.de/blog/timeline.html?filter=series::libnix"
   >libnix</a
 >
</p>
<p>we also cover these topics:</p>
<ul>
 <li>libnix: <strong>why we picked MinGW</strong> vs. other solutions</li>
 <li>general roadmap</li>
</ul>
<h1 id="libnix-mingw-vs.-other-solutions">
 libnix: MinGW vs. other solutions
</h1>
<p>
 making <strong>nix work native on windows</strong> there are a few
 options, here are a few updates reaching out to these communities:
</p>
<h2 id="tvix">Tvix</h2>
<p>
 <a href="https://tvix.dev/">tvix</a> is a rust reimplementation of the c++
 nix implementation, recent news:
</p>
<ul>
 <li>
   <strong>store implementation</strong> since
   <a href="https://tvl.fyi/blog/tvix-update-february-24">last update</a>
 </li>
 <li>
   <strong>nix evaluation</strong> comes close to upstream c++ nix,
   however, still effort to get to 100%
 </li>
 <li>not all <strong>builtins</strong> are supported yet</li>
 <li>no builders yet</li>
 <li>
   could be used to
   <strong>replace the tour of nix emscripten based backend</strong> but
   not much more
 </li>
</ul>
<div class="alert alert-warning" role="alert">
 <p>
   <strong
     >too early for considering tvix to ‘building software using nix’ on
     windows</strong
   >. tvix is amazing, i hope one day this code base replaces the c++ one.
 </p>
</div>
<h2 id="cosmopolitan">cosmopolitan</h2>
<p>
 <a href="https://justine.lol/cosmopolitan/">cosmopolitan</a> developers on
 discord mentioned to me that they had tried porting nix with cosmopolitan:
</p>
<blockquote>
 <p>
   ariel nunez: Bash on windows was only possible last year, after a lot of
   work by jart and contributors, and now Windows Terminal Preview can use
   it. I read my logs and last attempt to compile Nix was on August 2023,
   at that point in time we found out Nix used Boost and that was a blocker
   at the time.
 </p>
</blockquote>
<div class="alert alert-warning" role="alert">
 <p>
   the <strong>cosmopolitan idea</strong> has a lot of potential. i’m
   uncertain of OS specific traits and how well they map to this POSIX
   generalization. for instance, when normalizing paths with
   std::filesystem it is decided on compile time for which platform the
   paths resolve. cosmopolitan runs on all systems, so std::filesystem
   would have to make this choice a runtime resolution instead.
 </p>
 <p>
   <strong
     >but for now i probably follow the john ericson / volth path with
     MinGW.</strong
   >
 </p>
</div>
<h2 id="mingw">MinGW</h2>
<p>
 <a href="https://en.wikipedia.org/wiki/MinGW">MinGW</a> cross compiler
 setup:
</p>
<ul>
 <li>
   using <code>mingw cross compiler from nixos-wsl</code> to build nix for
   windows
 </li>
 <li>
   john ericson’s MinGW contributions are ongoing and promising ~early 2024
 </li>
</ul>
<div class="alert alert-warning" role="alert">
 <p>
   the
   <a href="https://www.mingw-w64.org/" class="uri"
     >https://www.mingw-w64.org/</a
   >
   toolchain is neatly done! in particular we want to use
   <a href="https://github.com/mstorsjo/llvm-mingw" class="uri"
     >https://github.com/mstorsjo/llvm-mingw</a
   >
   instead of gcc/ld.
 </p>
 <p>so <strong>lld/clang</strong> will be used:</p>
 <ul>
   <li>to build nix</li>
   <li>nixpkgs toolchain to build c/c++ programs for windows</li>
 </ul>
</div>
<h1 id="libnix-general-roadmap">libnix: general roadmap</h1>
<p>
 here is a list of things which need to be done still, see
 <a href="https://github.com/NixOS/nix/labels/windows" class="uri"
   >https://github.com/NixOS/nix/labels/windows</a
 >
 for detailed tickets.
</p>
<h2 id="meson-build-system">1. meson build system</h2>
<ul>
 <li>
   in order to build nix on windows natively, we need a build system which
   is not tied to bash. therefore meson is a good candidate for this and
   there have been a couple of patches already.
 </li>
</ul>
<h2 id="create-test-suite-for-nix-on-windows">
 2. create test suite for nix on windows
</h2>
<ul>
 <li>adapt unix specific tests to work on windows</li>
 <li>
   write windows specific tests for symlinks / path length / permissions
   and such
 </li>
 <li>
   run them in <a href="https://winehq.org">wine</a> / docker windows
   <a href="https://www.youtube.com/watch?v=xhGYobuG508" class="uri"
     >https://www.youtube.com/watch?v=xhGYobuG508</a
   >
 </li>
</ul>
<h2 id="assemble-prototype-windows-bootstrap-system">
 3. assemble prototype windows bootstrap system
</h2>
<ul>
 <li>
   make <code>nix</code> evaluate on windows use the store on
   <code>c:\</code> and
 </li>
 <li>
   get <code>runProgram</code> working to execute tools like
   <code>git</code> for <code>fetchUrl</code>
 </li>
 <li>use third-party built tools from MSYS2 (not built by nix)</li>
</ul>
<h2 id="build-hello-world-nixpkgs-win">
 4. build ‘hello world’ nixpkgs-win
</h2>
<ul>
 <li>
   <p>minimal nixpkgs like abstraction</p>
   <ul>
     <li>
       instead of trying to adapt nixpkgs we should start small with our
       own <code>stdenv</code> with <code>mingw</code> to show how to use
       it
     </li>
   </ul>
 </li>
</ul>
<h2 id="make-nix-toolchain-build-from-windows">
 5. make nix + toolchain build from windows
</h2>
<ul>
 <li>use the prototype toolchain to built itself</li>
 <li>
   adapt <code>bash</code> and unix favoring build systems into a windows
   world
 </li>
</ul>
<h2 id="nix-installer-channel">6. nix installer / channel</h2>
<ul>
 <li>
   <p>create an <strong>installer for nix</strong></p>
   <p>
     <a
       href="https://nixos.org/manual/nix/stable/installation/upgrading"
       class="uri"
       >https://nixos.org/manual/nix/stable/installation/upgrading</a
     >
   </p>
 </li>
 <li>
   <p>create a nix <strong>channel</strong></p>
   <ul>
     <li>create a channel for windows</li>
   </ul>
 </li>
</ul>
<h2 id="cargo-with-nix-support">7. cargo with nix support</h2>
<ul>
 <li>
   we start with <code>rust/cargo</code> on linux to make it work with nix
   as backend, see
   <a href="https://github.com/NixOS/nix/pull/8699" class="uri"
     >https://github.com/NixOS/nix/pull/8699</a
   >
   for this
 </li>
</ul>
<h1 id="libnix-future-nix-work">libnix: future nix work</h1>
<p>
 these items need to be done outside of the
 <a href="https://nlnet.nl/project/libnix/">libnix</a> funding but are
 still worth mentioning.
</p>
<h2 id="sandboxing">1. sandboxing</h2>
<ul>
 <li>
   process isolation in windows, for sandboxing <code>nix-build</code>, see
   <a
     href="https://learn.microsoft.com/de-de/virtualization/windowscontainers/manage-containers/hyperv-container#process-isolation"
     class="uri"
     >https://learn.microsoft.com/de-de/virtualization/windowscontainers/manage-containers/hyperv-container#process-isolation</a
   >
 </li>
</ul>
<h2 id="user-environments-pure-powershell-environment">
 2. user environments (pure powershell environment)
</h2>
<ul>
 <li>
   <code>source $HOME/.nix-profile/etc/profile.d/nix.sh</code> for
   powershell
 </li>
</ul>
<h2 id="nix-daemon-multi-user-mode">3. nix-daemon &amp; multi user mode</h2>
<ul>
 <li>
   <p>nix-daemon</p>
   <ul>
     <li>calls <code>nix-build</code> (with different UID/GID)</li>
     <li>
       windows <strong>unix domain socket support</strong> can be used
     </li>
   </ul>
 </li>
</ul>
<h2 id="nixos-module-system-systemd-like-windows-abstraction">
 4. nixos module system <code>systemd</code> like windows abstraction
</h2>
<ul>
 <li>
   the nixos module system creates <code>systemd</code> targets on linux
   and it would be nice if we had something similar for windows, see
   <a
     href="https://www.reddit.com/r/selfhosted/comments/8ijs26/systemdlike_to_create_windows_services_from/"
     >systemd equivalent on windows</a
   >
 </li>
</ul>
<h2 id="store-interoperability">5. store interoperability</h2>
<ul>
 <li>think <code>/nix/store</code> vs. <code>c:\nix\store</code></li>
 <li>
   <a href="https://github.com/NixOS/nix/issues/9205" class="uri"
     >https://github.com/NixOS/nix/issues/9205</a
   >
   - Use std::filesystem::path for Path
 </li>
 <li>
   <a href="https://github.com/NixOS/nix/issues/3197" class="uri"
     >https://github.com/NixOS/nix/issues/3197</a
   >
   - Encoding store Paths on Windows and Unix
 </li>
</ul>
<h2 id="usability-documentation">6. usability &amp; documentation</h2>
<ul>
 <li>support <code>nix repl</code></li>
 <li>support <code>man pages</code> in <code>nix build --help</code></li>
 <li>…</li>
</ul>
<h1 id="summary">summary</h1>
<p>
 this is a
 <strong
   >short summary of libnix topics what we are aiming for till the end of
   2024</strong
 >.
</p>
<p>
 additional we think that <strong>llvm-mingw</strong> is a potent toolchain
 for windows which enables us to use
 <a href="https://github.com/nixcloud/fixPath">fixPath</a>.
</p>
    
    </div>"#;
    log::info!("Example1");

    let old_node: Node<()> = parse_html::<()>("").unwrap().unwrap();
    log::info!("Example2");

    let new_node: Node<()> = raw_html::<()>(new_html);
    log::info!("Example3");

    let root = dom::create_dom_node(&old_node, ev_callback);
    log::info!("Example4");

    let root_node = Rc::new(RefCell::new(Some(root)));
    log::info!("Example5");

    let vdom_patches = vdom::diff(&old_node, &new_node).unwrap();
    log::debug!("Created {} VDOM patch(es)", vdom_patches.len());
    log::debug!("Created {:?}", vdom_patches);

    // convert vdom patch to real dom patches
    let dom_patches = dom::convert_patches(
        &root_node.borrow().as_ref().unwrap(),
        &vdom_patches,
        ev_callback,
    )
    .unwrap();
    log::debug!("Converted {} DOM patch(es)", dom_patches.len());
    log::debug!("Converted {:?}", dom_patches);



    dom::apply_dom_patches(root_node, mount_node, dom_patches).unwrap();
}
