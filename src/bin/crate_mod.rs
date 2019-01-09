// 这个crate的名字定义在Cargo.toml的[lib]段中
extern crate mylib;

use mylib::bar::foo_bar::foo_bar;
use mylib::foo::hello_world;

fn main() {
    println!("{}", hello_world());
    println!("{}", foo_bar());
}
