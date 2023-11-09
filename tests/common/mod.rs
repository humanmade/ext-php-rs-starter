use rand::Rng;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;
use std::thread::spawn;
use std::{env, fs};
use tokio::{io, net::TcpStream};

use fastcgi_client::{Client, Params, Request};

static BUILD: Once = Once::new();

fn setup_php_fpm() {
    Command::new("pkill").args(["-f", "php-fpm"]).output().unwrap();
    let fpm_path = String::from_utf8(
        Command::new("which")
            .args(["php-fpm"])
            .output()
            .unwrap_or_else(|_e| panic!("Could not find php-fpm"))
            .stdout,
    )
    .unwrap();

    spawn(move || {
        let _result = Command::new(fpm_path.trim_end())
            .arg(format!(
                "-dextension={}/target/debug/lib{}.{}",
                env::current_dir().unwrap().to_str().unwrap(),
                env!("CARGO_PKG_NAME"),
                std::env::consts::DLL_EXTENSION
            ))
            // Add other arguments here!
            // .stderr(Stdio::inherit())
            // .stdout(Stdio::inherit())
            .output()
            .unwrap();
    });
}

pub fn setup() {
    BUILD.call_once(|| {
        assert!(Command::new("cargo")
            .arg("build")
            .output()
            .expect("failed to build extension")
            .status
            .success());
    });

    setup_php_fpm();
}

pub fn write_test_file(script_name: &str, code: &str) -> PathBuf {
    let script_filename = env::current_dir().unwrap().join("tests/temp").join(script_name);
    fs::write(script_filename.clone(), code).unwrap();
    script_filename
}

pub fn fcgi_request(code: &str) -> String {
    let rand_name = rand::thread_rng().gen_range(1..99999999).to_string() + ".php";
    let script_name = rand_name.as_str();
    let script_filename = write_test_file(&script_name, code);
    println!("{}", script_filename.display());

    let res = fcgi_request_file(script_name, script_filename.to_str().unwrap());
    fs::remove_file(script_filename).unwrap();

    res
}

pub fn fcgi_request_file(script_name: &str, script_filename: &str) -> String {
    let params = Params::default()
        .request_method("GET")
        .script_name(script_name)
        .script_filename(script_filename)
        .request_uri(script_name)
        .document_uri(script_name)
        .remote_addr("127.0.0.1")
        .remote_port(12345)
        .server_addr("127.0.0.1")
        .server_port(80)
        .server_name("rust-tests")
        .content_type("")
        .content_length(0);

    // Fetch fastcgi server(php-fpm) response.
    let mut output = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let stream = TcpStream::connect(("127.0.0.1", 9000)).await.unwrap();
            let client = Client::new(stream);
            client.execute_once(Request::new(params, &mut io::empty())).await
        })
        .unwrap();
    output.stdout = output.stdout.map(|r| {
        let str = String::from_utf8(r).unwrap();
        let parts = str.split("\r\n\r\n");
        let content = parts.last().unwrap_or("").as_bytes().to_vec();
        content
    });

    String::from_utf8(output.stdout.unwrap()).unwrap()
}
