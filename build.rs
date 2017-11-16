extern crate peg;

fn main() {
    peg::cargo_build("src/parser/ecma-262.rustpeg");
}