# ext-php-rs-starter

A starter project for building PHP extensions with Rust, using [ext-php-rs](https://github.com/davidcole1340/ext-php-rs).

Note: This currently uses [our fork of ext-php-rs](https://github.com/joehoyle/ext-php-rs) - we are working to reintegrate this with mainline.


## Included files

* `.github/workflows/build.yml` - GitHub Actions workflow for building your extension automatically on push.
* `benches/` - Benchmark framework for `cargo bench`, using php-cli
* `integration-test/` - Docker setup for building and running an end-to-end integration test, using php-fpm + nginx.
* `src/` - Minimal skeleton with stubs for phpinfo and common module setup
* `tests/`
	* `common/` - Helpers for setting up tests
	* `example.rs` - Example of using the helpers
* Other files: common tools


## License

This repository is released into the public domain, and is marked with [CC0 1.0](http://creativecommons.org/publicdomain/zero/1.0). Human Made disclaims all applicable copyright to code in this repository.
