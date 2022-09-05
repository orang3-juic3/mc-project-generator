use std::rc::Rc;
use crate::cli::Cli;
use std::path::{Path, PathBuf};
use std::env;
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::Read;

pub struct Gradle {
    cli: Rc<Cli>,
    path: PathBuf
}

impl Gradle {
    pub fn new(cli: Rc<Cli>) -> Self {
        if let Some(gr) = &cli.gradle_dist {
            let mut binary = gr.clone();
            if binary.is_dir() {
                #[cfg(not(target_os = "windows"))]
                let gr_str = "gradle";
                #[cfg(target_os = "windows")]
                let gr_str = "gradle.bat";
                binary.push(gr_str);
                if binary.exists() && binary.is_file() {
                    /*dbg!(binary.as_os_str());
                    let mut child = Command::new(binary.as_os_str())
                                             .stdout(Stdio::piped())
                                             .arg("--no-daemon").spawn().unwrap();
                    println!("{}",String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap());*/
                    return Self {
                        cli: Rc::clone(&cli),
                        path: binary
                    }
                }
            } else {
                panic!(r"Invalid gradle dist directory provided!
                 (Example of correct (Windows) directory: F:\\gradle\\dists\\gradle-6.8-bin\\gradle-6.8\\bin\\)")
            }
        }
        let wait = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "gradle --no-daemon -v"]).stdout(Stdio::piped()).spawn()
        } else {
            Command::new("sh").args(["-c", "gradle --no-daemon -v"]).stdout(Stdio::piped()).spawn()
        };
        dbg!(&wait);
        if let Ok(mut child) = wait {
        /*if let Some(paths) = env::var_os("PATH") {
            let gradle : Vec<PathBuf> = env::split_paths(&paths).filter(|it| {
                    let mut sep = String::from(std::path::MAIN_SEPARATOR);
                    if sep == r"\" {
                        sep.push('\\');
                    }
                    let regex_str = format!(r".+{}gradle-(?P<ver>[\d.]+)-[^{}]+{}bin{}?", sep,sep,sep,sep); // wtf
                    let r = Regex::new(&*regex_str).unwrap();
                    true
                }).collect();
            if !gradle.is_empty() {
                return Self {
                    cli: Rc::clone(&cli),
                    path: gradle[0].clone()
                }
            }*/ // Why do regex garbage when you can just do the command?
            dbg!("tyt");
            println!("{}", String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap());
        }

        return Self {
            cli : Rc::clone(&cli),
            path : PathBuf::new()
        }
    }

}