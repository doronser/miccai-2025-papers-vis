# Testing Guidelines

When creating tests follow these guidelines:

* assume only the dst repo is available. if you require any data from `/l2l/src`, make sure it is committed to the repo in `/l2l/dst/`

* always use relative paths. the test should be runnable from any environment and deployment

* clearly mark which test are a rust version of existing tests and which are new tests that did not previously exist:

  * for tests that were translated, add a comment at top of file stating this it.

  * for new tests, add a comment at top of file explaining which part of the code covered by these tests.

<br />

When running and fixing tests follow these guidelines:

* `cargo test` stops running after the first crate that has failing tests. to run all test exhaustively use `cargo test --no-fail-fast`

* use `cargo test -p` to test specific crates

* after all fixes are applied, always run `cargo test --no-fail-fast` to verify you didn't break anything else

