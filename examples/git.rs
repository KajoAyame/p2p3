extern crate git2;

use git2::{Repository, Error};
use std::io;


fn main() {
    /*
    use std::io::Write; // For flush().

    print!("> ");
    assert!(io::stdout().flush().is_ok());

    // Get the command line from user input
    let mut url = String::new();
    assert!(io::stdin().read_line(&mut url).is_ok());

    let u = url.as_str();
    */

    let git = GitAccess::new("https://github.com/KajoAyame/p2p3");

    /*
    match git.clone("~/temp/p2p3") {
        Ok(()) => {
            println!("OK");

        }
        Err(e) => {
            println!("{}", e)
        }
    }*/
    //io::println(fmt!("%?", result));
 }

 #[derive(Clone,PartialEq,Debug)]
 pub struct GitAccess {
     repo_url: &'static str,
 }

 impl GitAccess {
     pub fn new(repo: &'static str) -> GitAccess {
         GitAccess{repo_url: repo}
     }

     pub fn clone(&self, dst_dir: &str) -> Result<(), git2::Error> {
         match Repository::clone(self.repo_url, dst_dir) {
             Ok(repo) => repo,
             Err(e) => return Err(e)
         };
         Ok(())
     }
     /*
     pub fn commit_path(&self, commit_message: &str, file_path: &str) -> Result<(), Error>  {
         let repo = match Repository::open(self.repo_url) {
             Ok(repo) => repo,
             Err(e) =>return Err(e)
         };
         let sig = try!(repo.signature());
         let tree_id = {
             let mut index = try!(repo.index());
             try!(index.add_path(Path::new(file_path)));
             try!(index.write_tree_to(&repo))
         };

         let tree = try!(repo.find_tree(tree_id));
         // lookup current HEAD commit
         let head_ref = match repo.head() {
             Ok(head_ref) =>  head_ref,
             Err(e) => return Err(e)
         };
         let head_oid = head_ref.target().unwrap();
         let commit = try!(repo.find_commit(head_oid));
         // make that parent of new commit
         try!(repo.commit(Some("HEAD"), &sig, &sig, commit_message, &tree, &[&commit]));
         Ok(())
     }

     pub fn push(&self) -> Result<(), git2::Error> {
         let repo = match Repository::open(self.repo_url) {
             Ok(repo) => repo,
             Err(e) => return Err(e)
         };

         let mut cb = RemoteCallbacks::new();
         cb.credentials(|_, _, _| {  // |repoName, options, cred_type|
             // get credentials from user
             Cred::userpass_plaintext(self.username, self.password)
         });
         let remote = "origin";
         let mut remote = try!(repo.find_remote(remote));
         let mut opt_push = PushOptions::new();
         opt_push.remote_callbacks(cb);
         let x: Option<&mut PushOptions> = Some(&mut opt_push);
         match remote.push(&["refs/heads/master"], x) {
             Ok(p) => p,
             Err(e) => return Err(e)
         };

         Ok(())
     }*/
 }
