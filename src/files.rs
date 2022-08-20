use crate::cli;
use std::ops::Deref;
use crate::cli::Cli;

struct CodeGen {
    settings_gradle: Option<&str>
    cli: Cli,
}
impl CodeGen {
    fn settings_gradle(&self) -> String {
        let mut skeleton = String::from(include_str!("skeleton/settings.gradle.kts"));
        skeleton.push_str(format!("{}", self.cli.name).as_str());
        return skeleton;
    }
}