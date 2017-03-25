use std::env;
use std::path::Path;
use std::process::{Stdio, Command};

const LIBCACA_REPO: &'static str = "https://github.com/cacalabs/libcaca";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir);

    setup();
    bootstrap();
    configure();
    make();
    install(&dst);
    clean();
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=caca");
}

fn setup() {
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg(LIBCACA_REPO);
    cmd.arg(".libcaca");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    cmd.current_dir(&cargo_dir);

    run(&mut cmd);
}

fn bootstrap() {
    let mut cmd = caca_cmd("./bootstrap");
    run(&mut cmd);
}

fn clean() {
    let mut cmd = Command::new("rm");
    cmd.arg("-rf");
    cmd.arg(".libcaca");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    cmd.current_dir(&cargo_dir);
    run(&mut cmd);
}

fn configure() {
    let mut cmd = caca_cmd("./configure");
    // Apparently, cfg! can't take a variable name...
    if cfg!(feature = "enable-ncurses") {
        cmd.arg("--enable-ncurses");
    }
    if cfg!(feature = "enable-slang") {
        cmd.arg("--enable-slang");
    }
    if cfg!(feature = "enable-conio") {
        cmd.arg("--enable-conio");
    }
    if cfg!(feature = "enable-x11") {
        cmd.arg("--enable-x11");
    }
    if cfg!(feature = "enable-gl") {
        cmd.arg("--enable-gl");
    }
    if cfg!(feature = "enable-win32") {
        cmd.arg("--enable-win32");
    }
    if cfg!(feature = "enable-network") {
        cmd.arg("--enable-network");
    }

    cmd.arg("--disable-csharp");
    cmd.arg("--disable-java");
    cmd.arg("--disable-cxx");
    cmd.arg("--disable-python");
    cmd.arg("--disable-ruby");
    // cmd.arg("--disable-dynamic");
    // cmd.arg("--enable-static");

    // Make the lib folder appear directly in the './out' directory, instead of
    // './usr/out' 
    cmd.arg("--prefix=/");

    let target = env::var("TARGET").unwrap();
    let cflags;
    if target.contains("i686") {
        cflags = "-m32"
    } else if target.contains("x86_64") {
        cflags = "-m64 -fPIC"
    } else {
        cflags = "-fPIC"
    }
    println!("configure: setting CFLAGS to: `{}`", cflags);
    env::set_var("CFLAGS", cflags);

    run(&mut cmd);
    env::remove_var("CFLAGS");
}

fn make() {
    let mut cmd = caca_cmd("make");
    run(&mut cmd);
}

fn install(dst: &Path) {
    let mut cmd = caca_cmd("make");
    cmd.arg(format!("DESTDIR={}", dst.display()));
    cmd.arg("install");
    run(&mut cmd);
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());
}

fn caca_cmd(cmd: &str) -> Command {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    let libcaca_dir = cargo_dir.join(".libcaca");
    let mut cmd = Command::new(cmd);
    cmd.current_dir(&libcaca_dir);
    cmd
}

