# P2P3 #

## Install for development ##
* Rust version 1.8
* Atom editor

## Install the following packages in Atom: ##
* build
* build-cargo
* cargo-test-runner
* language-rust
* linter
* linter-rust
* racer

## Configure racer ##
Use Cargo to install racer (an auto-complete program for rust) by running "cargo install racer" on the command line or terminal.

Download and extract the rust 1.8 source somewhere. In the racer settings you need to say where the racer executable is and where the rust source root directory is. On windows the racer executable should be in "%USER_HOME%\.cargo\bin\racer.exe", and on unix-like systems it will be "/usr/local/bin/racer". It will also let you know where racer is after cargo finishes installing it.

If we all follow the above we should have a common set of tools to reason about when communicating. Also, it should be more convenient than plain-text editing.