#![allow(dead_code)]

use storage::storage_helper::GitAccess;
use git2::ErrorCode;

pub enum PermissionLevel {
    Editor,
    Viewer
}

pub fn get_permission_level(ga: &GitAccess) -> PermissionLevel {
    return match ga.push() {
        Ok(()) => PermissionLevel::Editor,
        Err(e) => {
            if e.code() == ErrorCode::GenericError {
                if e.to_string().contains("403") {
                    println!("User does not have write access")
                } else if e.to_string().contains("401") {
                    println!("Invalid password")
                }
            } else if e.code() == ErrorCode::NotFastForward {
                match ga.pull_repo() {
                    Ok(()) => println!("pull repo successful"),
                    Err(e) => println!("pull repo error {}", e),
                };
                return get_permission_level(ga);
            }
            PermissionLevel::Viewer
        },
    };
}
