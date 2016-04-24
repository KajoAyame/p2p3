extern crate gcc;

use std::process::Command;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::path::PathBuf;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone)]
pub enum CompileMode {
    None,
    C,
    Python,
    Ruby,
    Lua,
}

impl FromStr for CompileMode {
    type Err = ();

    fn from_str(s: &str) -> Result<CompileMode, ()> {
        match s {
            "c_cpp" => Ok(CompileMode::C),
            "python" => Ok(CompileMode::Python),
            "ruby" => Ok(CompileMode::Ruby),
            "lua" => Ok(CompileMode::Lua),
            _ => Ok(CompileMode::None),
        }
    }
}

fn make_temp_dir() -> PathBuf {
    let mut temp_dir_name = env::temp_dir();
    temp_dir_name.push("P2P3");
    fs::create_dir(temp_dir_name.as_path()).unwrap_or( () );
    //Return
    temp_dir_name
}

fn make_file(path: &Path, input: &str) -> Result<(), String>{
    let mut file = File::create(&path).unwrap_or_else(|e| panic!("Oh noooooo {}", e));
    match file.write_all(&input.as_bytes()){
        Err(why)=>{
            let display = path.display();
            return Err(format!("couldn't write to {}: {}", display,
                                               Error::description(&why)));
        },
        Ok(_)=>Ok( () )
    }
}

#[allow(dead_code)]
pub fn run_code(compile_mode: CompileMode, input: &str) -> Result<String, String> {
    match compile_mode {
        CompileMode::C => run_c(input),
        CompileMode::Python => run_python(input),
        CompileMode::Lua => run_lua(input),
        CompileMode::Ruby => run_ruby(input),
        CompileMode::None => return Err("Could not find suitable compiler".to_string()),
    }
}


#[allow(dead_code)]
pub fn run_c(input: &str) -> Result<String, String> {
    println!("run c code {}", input);
    let temp_dir_name = make_temp_dir();
    let c_file = temp_dir_name.join("temp.c");
    let exe = temp_dir_name.join("temp.exe");

    match make_file(&c_file, &input){
        Ok(_) =>{},
        Err(e) => return Err(e)
    };

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

#[allow(dead_code)]
pub fn run_python(input: &str) -> Result<String, String> {
    println!("run python code");
    run_interp(input, "python")
}

#[allow(dead_code)]
pub fn run_ruby(input: &str) -> Result<String, String> {
    println!("run ruby code");
    run_interp(input, "ruby")
}

#[allow(dead_code)]
pub fn run_lua(input: &str) -> Result<String, String> {
    println!("run lua code");
    run_interp(input, "lua")
}

#[allow(dead_code)]
fn run_interp(input: &str, cmd: &str) -> Result<String, String> {
    let temp_dir = make_temp_dir();
    let tmp_file = temp_dir.join(format!("temp.{}", cmd));

    match make_file(&tmp_file, &input){
        Ok(_) =>{},
        Err(e) => return Err(e)
    };

    match Command::new(cmd).current_dir(temp_dir).arg(&tmp_file).output(){
        Ok(out) => {
            if out.status.success() {
                Ok(String::from_utf8(out.stdout).unwrap())
            }else{
                let a = out.status.code().unwrap();
                let b = String::from_utf8(out.stderr).unwrap();
                let c = String::from_utf8(out.stdout).unwrap();
                Err(format!("{} failed with code {}: {} {}",cmd, a, b, c))
            }
        },
        Err(err) => Err(format!("Failed to run python with error: {}",err))
    }
}

#[cfg(test)]
mod test{
    use super::*;

    static C_CODE: &'static str =
"#include <stdio.h>

int main(){
    printf(\"Hello World\");
    return 0;
}";

    static PY_CODE: &'static str = "print \"Hello World\",";

    static LUA_CODE: &'static str = "print(\"Hello World\")";

    static RUBY_CODE: &'static str = "puts \"Hello World\" ";

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

    #[test]
    fn run_simple_py(){
        let out = run_python(PY_CODE);

        print!("{:?}", out);
        match out{
            Err(_) => { assert!(false) },
            Ok(res) => {
                assert_eq!(res.trim_right(), "Hello World");
            }
        }
    }

    #[test]
    fn run_simple_lua(){
        let out = run_lua(LUA_CODE);

        print!("{:?}", out);
        match out{
            Err(_) => { assert!(false) },
            Ok(res) => {
                assert_eq!(res.trim_right(), "Hello World");
            }
        }
    }

    #[test]
    fn run_simple_ruby(){
        let out = run_ruby(RUBY_CODE);

        print!("{:?}", out);
        match out{
            Err(_) => { assert!(false) },
            Ok(res) => {
                assert_eq!(res.trim_right(), "Hello World");
            }
        }
    }


}
