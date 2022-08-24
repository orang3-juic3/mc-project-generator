use crate::cli;
use crate::cli::Cli;
use regex::Regex;
use chrono::{DateTime, NaiveDateTime, ParseResult, ParseError};
use std::cmp::Ordering;
use chrono::format::ParseErrorKind;
use std::rc::Rc;

pub struct CodeGen {
    settings_gradle: Option<String>,
    build_gradle: Option<String>,
    release_ver: Option<String>,
    cli: Rc<Cli>,
}
impl CodeGen {
    pub fn from(cli: Rc<Cli>) -> Self {
        Self {
            settings_gradle:None,
            build_gradle: None,
            release_ver: None,
            cli,
        }
    }

    pub fn settings_gradle(&mut self) -> &str {
        if let None = self.settings_gradle {
            let mut skeleton = String::from(include_str!("skeleton/settings.gradle.txt"));
            skeleton.push_str(format!("{}", self.cli.name).as_str());
            self.settings_gradle = Some(skeleton);
        }
        self.release_ver.as_ref().unwrap()
    }

    pub fn release_ver(&mut self) -> &str{
        if let None = self.release_ver {
            let r = Regex::new(r"\d+\.(?P<v>\d+)(\.\d+(-SNAPSHOT)?)?").unwrap();
            let ver : u16 = r.captures(&self.cli.version).unwrap().name("v").unwrap().as_str().parse().unwrap();
            let url = if ver < 17 {"https://papermc.io/repo/service/rest/repository/browse/maven-public/com/destroystokyo/paper/paper-api/"}
            else {"https://repo.papermc.io/service/rest/repository/browse/maven-public/io/papermc/paper/paper-api/"};
            let mut content = String::new();
            let vers = CodeGen::retrieve_api_versions(url, &mut content).into_iter().filter(|it| {
                let x = it.split("-").next();
                if let Some(v) = x {
                    if v == self.cli.version {
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

    fn retrieve_api_versions<'a>(url: &'a str, content: &'a mut String) -> Vec<&'a str>{
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

}