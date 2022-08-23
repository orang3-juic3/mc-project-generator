use crate::cli;
use crate::cli::Cli;
use regex::Regex;
use chrono::{DateTime};

pub struct CodeGen {
    settings_gradle: Option<String>,
    build_gradle: Option<String>,
    release_ver: Option<String>,
    cli: Box<Cli>,
}
impl CodeGen {
    pub fn from(cli: Box<Cli>) -> Self {
        Self {
            settings_gradle:None,
            build_gradle: None,
            release_ver: None,
            cli,
        }
    }
    pub fn settings_gradle(&mut self) -> &str {
        if let None = self.settings_gradle {
            let mut skeleton = String::from(include_str!("skeleton/settings.gradle.kts"));
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
            });
            // write regex for directory listings
        }
        ""
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

    fn retrieve_newest_version<'a>(url : &'a str, mut matching: Vec<&'a str>) -> &'a str {
        matching.sort_by(|a,b| {
            // regex date time out of link reformat to create date time then compare
        });
        ""
    }

}