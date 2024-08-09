use std::env;
use std::path::PathBuf;

use git2::{self, Cred, RemoteCallbacks, Repository};

fn main() {
    let foo_ing = "https://github.com/helio-frota/plrust-example";
    let target = PathBuf::from("target/bar");

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
      Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
        None,
      )
    });
  
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
  
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    builder.clone(
        foo_ing,
        &target,
    ).unwrap();

    println!("see something inside target dir...");

}
