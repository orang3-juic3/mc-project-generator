use std::rc::Rc;
use crate::cli::Cli;
use std::path::{Path, PathBuf};
use std::env;
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::{Read, Error, ErrorKind};
use std::fs::DirEntry;
use chrono::format::Parsed;
use std::cmp::{Ordering, max};
use std::convert::TryFrom;
use std::string::ParseError;
use std::num::ParseIntError;

pub struct Gradle {
    cli: Rc<Cli>,
    pub path: PathBuf
}

impl Gradle {
    pub fn new(cli: Rc<Cli>) -> Self {
        if let Some(gr) = &cli.gradle_dist {
            let mut binary = gr.clone();
            if binary.is_dir() {
                let gr_str = Self::gradle_exec_name();
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
            if let Ok(output)  = child.wait_with_output() {
                if output.status.code().is_some() && output.status.code().unwrap() == 0 {
                    let mut path = PathBuf::new();
                    path.push("gradle");
                    return Self {
                        cli: Rc::clone(&cli),
                        path
                    }
                }
            }
        }
        let mut gradles = dirs::home_dir().unwrap();
        gradles.push(".gradle");
        gradles.push("wrapper");
        gradles.push("dists");
        let re = Self::compile_gradle_regex();
        if gradles.is_dir() {
            type GradleIO = (Result<SemVer, std::io::Error>, PathBuf);
            let mut dirs : Vec<GradleIO>= gradles.read_dir().unwrap().filter_map(|it| {
                if it.is_ok() {
                    return Some(it.unwrap())
                }
                None
            }).filter_map(|it| {
                let str_name = it.file_name();
                dbg!(&str_name);
                let x = re.captures(str_name.to_str()?).and_then(|cap| {
                    dbg!(&cap.name("ver")?.as_str());
                    Some(SemVer::try_from(cap.name("ver")?.as_str()))
                })?;
                Some((x, it.path()))
            }).collect();
            let mut sem_vers: Vec<(SemVer, PathBuf)>= Vec::new();
            for i in dirs {
                if i.0.is_err() {
                    panic!("{}",i.0.err().unwrap())
                }
                sem_vers.push((i.0.unwrap(), i.1));
            }
            sem_vers.sort_by(|a,b| {
                a.0.partial_cmp(&b.0).unwrap()
            });
            dbg!(&sem_vers);
            if sem_vers.last().is_some() {
                let path = sem_vers.pop().unwrap().1;
                return Self {
                    cli : Rc::clone(&cli),
                    path
                }
            }
        }
        panic!("Could not find a gradle implementation!")
    }

    fn compile_gradle_regex() -> Regex {
        Regex::new(r"gradle-(?P<ver>[\d.]+)-(?:.+-)?(?:bin|all)").unwrap()
    }

    pub fn gradle_exec_name() -> &'static str {
        #[cfg(not(target_os = "windows"))]
        return "gradle";
        #[cfg(target_os = "windows")]
        return "gradle.bat";
    }

}

#[derive(Debug)]
pub struct SemVer {
    vers: Vec<u16>,
}

impl TryFrom<&str> for SemVer {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vers_raw  : Vec<Result<u16, ParseIntError>>= value.split(".").map(|it| {
            it.parse::<u16>()
        }).collect();
        let mut vers = Vec::new();
        for i in vers_raw {
            if i.is_err() {
                return Err(Error::new(ErrorKind::InvalidInput, i.err().unwrap()))
            }
            vers.push(i.unwrap());
        }
        Ok(Self {
            vers
        })
    }
}

impl PartialEq<Self> for SemVer {
    fn eq(&self, other: &Self) -> bool {
        if let Some(order) = &self.partial_cmp(other) {
            return match order {
                Ordering::Less => { false }
                Ordering::Equal => { true }
                Ordering::Greater => { false }
            }
        }
        false
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let l1 = &self.vers;
        let l2 = &other.vers;
        for i in 0..max(l1.len(), l2.len()) {
            if i >= l1.len() {
                return Some(Ordering::Less);
            }
            if i >= l2.len() {
                return Some(Ordering::Greater);
            }
            if l1[i] > l2[i] {
                return Some(Ordering::Greater);
            }
            if l1[i] < l2[i] {
                return Some(Ordering::Less);
            } else {
                if l1.len() == l2.len() && i == (l1.len() -1) {
                    return Some(Ordering::Equal);
                }
            }
        }
        None
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less) || self.partial_cmp(other) == Some(Ordering::Equal)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater) || self.partial_cmp(other) == Some(Ordering::Equal)
    }
}