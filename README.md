# DiscountService In Rust On AWS

A simple discount service written in Rust made to run on AWS Lambda.


## Setup (on MacOS)

* Install Rust
* Run `brew install filosottile/musl-cross/musl-cross`
* Create symlink `ln -s /usr/local/opt/musl-cross/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc`
* Checkout code
* In eash of the sub directories run:
  * `rustup target add x86_64-unknown-linux-musl`
  * `sh build.sh`
* Upload code to AWS and set permissions accordingly


## Todo

* Deploy to AWS automatically


