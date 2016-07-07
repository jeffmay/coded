extern crate coded;

use coded::helpers;

/*
 * NOTE: stdout is captured by cargo, so println! does not print to console
 * automatically. To see println! output, run the tests with these flags:
 *
 *     $ cargo test -- --nocapture
 *
 * See: http://stackoverflow.com/questions/25106554/println-doesnt-work-in-rust-unit-tests
 */


#[test]
fn new_config_read() {
    let conf: helpers::Config = helpers::new_config("tests/conf.yml");
    println!("CONF DEBUG: {:?}", conf);
    assert_eq!(conf.dirs.len(), 2);
    assert_eq!(conf.dirs[0].as_str(), "~/Code");
}

