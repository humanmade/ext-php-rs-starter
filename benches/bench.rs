#![feature(test)]

use std::{
    env,
    io::Write,
    process::{Command, Stdio, Termination},
    sync::Once,
};
extern crate test;

use test::black_box;

static BUILD: Once = Once::new();

fn setup() {
    BUILD.call_once(|| {
        assert!(Command::new("cargo")
            .arg("build")
            .output()
            .expect("failed to build extension")
            .status
            .success());
    });
}

fn run_php(php: &str) -> Result<(), ()> {
    let mut child = Command::new("php")
        .arg(format!(
            "-dextension={}/target/debug/lib{}.{}",
            env::current_dir().unwrap().to_str().unwrap(),
            env!("CARGO_PKG_NAME"),
            std::env::consts::DLL_EXTENSION,
        ))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(php.as_bytes()).unwrap();
    if child.wait().unwrap().success() {
        Ok(())
    } else {
        Err(())
    }
}

// #[bench]
// fn test_some_thing(b: &mut test::Bencher) -> impl Termination {
//     setup();
//     b.iter(|| {
//         black_box(run_php(r#"
//                 <?php
//                 $foo = 'bar';
//             "#
//         ).unwrap() )
//     } )
// }
