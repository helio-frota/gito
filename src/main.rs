use std::path::PathBuf;

use git2::{self, Repository};

fn main() {
    let foo_ing = "https://github.com/helio-frota/plrust-example";
    let target = PathBuf::from("target/bar");
    let _foo_bar = Repository::clone(foo_ing, target);
    println!("see something inside target dir...");

}
