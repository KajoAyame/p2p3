extern crate gcc;

use std::process::Command;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[allow(dead_code)]
pub fn run_c(input: &str) -> Result<String, String> {
    let mut temp_dir_name = env::temp_dir();
    temp_dir_name.push("P2P3");
    let temp_dir_name = temp_dir_name.as_path();
    fs::create_dir(temp_dir_name).unwrap_or( () );
    let c_file = temp_dir_name.join("temp_c_file.c");
    let exe = temp_dir_name.join("temp.exe");

    {
        let mut file = File::create(&c_file).unwrap_or_else(|e| panic!("Oh noooooo {}", e));
        match file.write_all(&input.as_bytes()){
            Err(why)=>{
                let display = c_file.display();
                return Err(format!("couldn't write to {}: {}", display,
                                                   Error::description(&why)));
            },
            Ok(_)=>{
                // File written, so proceed
            }
        };
    }

    let output = match gcc::windows_registry::find("x86_64_msvc", "cl.exe"){
        Some(cc) => {
            let mut cc= cc;
            print!("{:?}", cc);
            cc.current_dir(temp_dir_name).arg(&c_file).arg(&format!("/link /OUT:{}", exe.to_str().unwrap())).output().unwrap() },
        None => Command::new("cc").current_dir(temp_dir_name).arg(&c_file).arg("-o").arg(&exe).output().unwrap()
    };

    if !output.status.success() {
        let a = output.status.code().unwrap();
        let b = String::from_utf8(output.stderr).unwrap();
        let c = String::from_utf8(output.stdout).unwrap();
        print!("Compile failed with code {}: {} {}", a, b, c);
        return Err(format!("Compile failed with code {}: {} {}", a, b, c));
    }

    let run_output = Command::new(exe).output().unwrap();

    if !run_output.status.success() {
        let a = run_output.status.code().unwrap();
        let b = String::from_utf8(run_output.stderr).unwrap();
        return Err(format!("Run failed with code {}: {}", a, b));
    }



    Ok(String::from_utf8(run_output.stdout).unwrap())
}

#[cfg(test)]
mod test{
    use super::*;

    static C_CODE: &'static str =
"#include <stdio.h>

void main(){
    printf(\"Hello World\");
    return 0;
}";

    #[test]
    fn run_simple_c(){
        let out = run_c(C_CODE);

        print!("{:?}", out);
        match out{
            Err(_) => { assert!(false) },
            Ok(res) => {
                assert_eq!(res, "Hello World");
            }
        }
    }
}
