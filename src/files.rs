use crate::cli;
use crate::cli::Cli;
use regex::Regex;
use chrono::{DateTime, NaiveDateTime, ParseResult, ParseError};
use std::cmp::Ordering;
use chrono::format::ParseErrorKind;
use std::rc::Rc;
use std::io::{Error, ErrorKind, Read, Write};
use colored::Colorize;
use std::borrow::Borrow;
use crate::gradlecreator::Gradle;
use std::process::{Command, Stdio};
use std::ffi::{OsStr, OsString};
use std::option::Option::Some;
use std::any::Any;
use std::sync::mpsc::{Sender, Receiver, RecvError};
use std::sync::mpsc;
use std::path::PathBuf;
use std::fs::File;
use tinytemplate::TinyTemplate;

pub struct CodeGen {
    settings_gradle: Option<&'static str>,
    build_gradle: Option<&'static str>,
    release_ver: Option<String>,
    cli: Rc<Cli>,
}
macro_rules! lazy {
        ($name : ident, $path: literal) => {
                pub fn $name(&mut self) -> &'static str {
                    if let None = self.settings_gradle {
                        let mut skeleton = include_str!($path);
                        self.settings_gradle = Some(skeleton);
                    }
                    self.$name.unwrap()
                }
        };
    }
impl CodeGen {
    pub fn from(cli: Rc<Cli>) -> Self {
        Self {
            settings_gradle: None,
            build_gradle: None,
            release_ver: None,
            cli,
        }
    }
    // le cheese
    lazy!(settings_gradle, "skeleton/settings.gradle.txt");

    lazy!(build_gradle, "skeleton/build.gradle.txt");

    pub fn release_ver(&mut self) -> &str {
        if let None = self.release_ver {
            let r = Regex::new(r"\d+\.(?P<v>\d+)(\.\d+(-SNAPSHOT)?)?").unwrap();
            let ver : u16 = r.captures(&self.cli.version).unwrap().name("v").unwrap().as_str().parse().unwrap();
            let url = if ver < 17 {"https://papermc.io/repo/service/rest/repository/browse/maven-public/com/destroystokyo/paper/paper-api/"}
            else {"https://repo.papermc.io/service/rest/repository/browse/maven-public/io/papermc/paper/paper-api/"};
            let mut content = String::new();
            let vers = CodeGen::retrieve_api_versions(url, &mut content).into_iter().filter(|it| {
                let x = it.split("-").next();
                if let Some(v) = x {
                    if v == &*self.cli.version {
                        return true;
                    }
                }
                false
            }).collect();
            let ver = Self::retrieve_newest_version(url, vers);
            self.release_ver = if let Some(a) = ver {
                Some(String::from(a))
            } else {
                panic!("Could not find matching dependency version for api version {}!", self.cli.version)
            };
        }
        self.release_ver.as_ref().unwrap()
    }

    fn retrieve_api_versions<'a>(url: &'a str, content: &'a mut String) -> Vec<&'a str> {
        content.push_str(&*reqwest::blocking::get(url).unwrap().text().unwrap());
        let r = Regex::new(r#"<td><a href="(?P<link>.+)/">(?P<v>.+)</a></td>"#).unwrap();
        let mut versions = Vec::new();
        for cap in r.captures_iter(&*content) {
            let link_ver = cap.name("link");
            let ver = cap.name("v");
            if ver.is_none() || link_ver.is_none() {
                continue;
            }
            let ver_str = ver.unwrap().as_str();
            if ver_str == link_ver.unwrap().as_str() {
                versions.push(ver_str);
            }
        }
        versions
    }

    fn retrieve_newest_version<'a, 't>(url : &'a str, matching: Vec<&'t str>) -> Option<&'t str> {
        let mut mapped : Vec<(&'t str, i64)> = matching.into_iter().map(|a| {
            let content = reqwest::blocking::get(format!("{}{}", url, a)).unwrap().text().unwrap();
            let r = Regex::new(r"<td>([a-zA-Z\d:\s]+)</td>").unwrap();
            let cap = r.captures_iter(&*content).next();
            if cap.is_none() {
                return (a, None);
            }
            //Sun Aug 21 03:07:41 UTC 2022 https://docs.rs/chrono/latest/chrono/format/strftime/
            let res = NaiveDateTime::parse_from_str(&cap.unwrap()[1].trim(),"%a %b %d %H:%M:%S %Z %Y");
            if res.is_err() {
                return (a, None);
            }
            (a, Some(res.unwrap()))
        }).filter_map(|it| {
            if it.1.is_none() {
                return None;
            }
            let date = it.1.unwrap();
            Some((it.0, date.timestamp()))
        }).collect();
        mapped.sort_by(|(_,a), (_,b)| {
            a.cmp(b)
        });
        return if let Some(a) = mapped.last() {
            Some(&*a.0)
        } else {
            None
        }

    }


    // bool = should_panic
    fn prompt_empty(&self) -> Result<(), (bool,std::io::Error)> {
        let path = &self.cli.dir;
        let overwrite = *&self.cli.overwrite;
        if !path.is_dir() && path.exists() {
            return Err((true, Error::new(ErrorKind::NotADirectory, format!("Target project path ({}) isn't a directory.", &self.cli.dir.to_string_lossy()))))
        }
        if path.exists()  {
            let entries = path.read_dir().unwrap().count();
            if entries > 0 {
                if !overwrite {
                    return Err((true, Error::new(ErrorKind::DirectoryNotEmpty, format!("Target project directory ({}) is not empty. Use the overwrite flag to ignore this.", &self.cli.dir.to_string_lossy()))));
                }
            }
        }
        Ok(())
    }

    pub fn gen_project(self, gradle: &mut Gradle) {
        let prompt_res = self.prompt_empty();
        if prompt_res.is_err() {
            let x = prompt_res.err().unwrap();
            if x.0 == true {
                panic!("{}", x.1);
            }
            println!("{}","Warning: Target directory is not empty, however override flag is set, so continuing..".yellow());
        }
        if self.cli.dir.exists() {
            std::fs::remove_dir_all(&self.cli.dir).unwrap();
        }
        std::fs::create_dir(&self.cli.dir).unwrap();
        Gradle::gradle_exec_name();
        let path = gradle.path.to_str();
        let mut cmd = OsString::new();
        #[cfg(target_os = "windows")]
        cmd.push("cd /d ");
        #[cfg(not(target_os = "windows"))]
        cmd.push("cd ");
        cmd.push(&self.cli.dir.as_os_str());
        cmd.push(" && ");
        if let Some(e) = path {
            if e == "gradle" {
                cmd.push("gradle");
            }
        } else {
            cmd.push(gradle.path.as_os_str());
        }
        cmd.push(" init --type basic --no-daemon");
        dbg!(&cmd);
        let cmd= cmd.as_os_str();
        let mut process = if cfg!(target_os = "windows") {
            Command::new("cmd").args([OsStr::new("/C"), cmd]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
        } else {
            Command::new("sh").args([OsStr::new("/C"), cmd]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
        }.unwrap();
        let mut stdin = process.stdin.take().unwrap();
        std::thread::spawn(move || {
            stdin.write_all(b"2").unwrap();
        });
        let mut str = String::new();
        process.stdout.take().unwrap().read_to_string(&mut str).unwrap();
        println!("{}", str.as_str());
        process.wait().unwrap();
        self.rm_gradle_file("build.gradle");
        self.rm_gradle_file("settings.gradle");
        let c = Rc::clone(&self.cli);
        self.template_gradle_files(c);
        ()
    }

    fn template_gradle_files(mut self, cli: Rc<Cli>) {
        let settings:&'static str = self.settings_gradle();
        let mut tt = TinyTemplate::new();
        tt.add_template("settings",settings);
        tt.render("settings",&cli.name).unwrap();
        let build = self.build_gradle();
    }

    fn rm_gradle_file(&self, file: &str) {
        let mut del_path = PathBuf::from(&self.cli.dir);
        del_path.push(file);
        std::fs::remove_file(del_path);
    }

    fn create_gradle_file(&self, name: &str, contents: &str) {
        let mut path = self.cli.dir.clone();
        path.push(name);
        let mut file = File::create(path).unwrap();
        file.write_all(contents.as_bytes());
    }

}