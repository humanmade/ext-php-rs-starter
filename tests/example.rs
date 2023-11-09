use indoc::indoc;
use std::thread::sleep;
use std::time;

mod common;
use common::fcgi_request;

fn setup() {
    common::setup();
    sleep(time::Duration::from_secs(1));
}

#[test]
fn test_some_thing() {
    setup();
    let output = fcgi_request(indoc! { r#"
            <?php
            var_dump( 'foo' );
        "#
    });
    assert_eq!(
        indoc! {r#"
            string(3) "foo"
        "#},
        output
    );
}
