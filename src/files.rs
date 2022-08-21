use crate::cli;
use crate::cli::Cli;
use regex::Regex;

pub struct CodeGen {
    settings_gradle: Option<String>,
    build_gradle: Option<String>,
    release_ver: Option<String>,
    cli: Cli,
}
impl CodeGen {
    pub fn from(cli: Cli) -> Self {
        Self {
            settings_gradle:None,
            build_gradle: None,
            release_ver: None,
            cli,
        }
    }
    pub fn settings_gradle(&mut self) {
        if let None = self.settings_gradle {
            let mut skeleton = String::from(include_str!("skeleton/settings.gradle.kts"));
            skeleton.push_str(format!("{}", self.cli.name).as_str());
            self.settings_gradle = Some(skeleton);
        }
    }
    pub fn release_ver(&mut self) {
        if let None = self.release_ver {
            let r = Regex::new(r"\d+\.(?P<v>\d+)(\.\d+(-SNAPSHOT)?)?").unwrap();
            let ver : u16 = r.captures(&self.cli.version).unwrap().name("v").unwrap().as_str().parse().unwrap();
            let url = if ver < 17 {"https://papermc.io/repo/service/rest/repository/browse/maven-public/com/destroystokyo/paper/paper-api/"}
            else {"https://repo.papermc.io/service/rest/repository/browse/maven-public/io/papermc/paper/paper-api/"};
            println!("{}", reqwest::blocking::get(url).unwrap().text().unwrap());
            // write regex for directory listings
        }
    }
}