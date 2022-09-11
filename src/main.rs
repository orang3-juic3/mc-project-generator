#![feature(io_error_more)]

mod files;
mod gradlecreator;

use clap::Parser;
use std::path::PathBuf;
use std::path::Path;
use crate::cli::Cli;
use crate::files::CodeGen;
use crate::gradlecreator::Gradle;
use std::rc::Rc;
use std::error::Error;
use colored::Colorize;

mod cli {
    use clap::Parser;
    use std::path::PathBuf;
    use std::path::Path;
    #[derive(Parser)]
    #[clap(name = "Minecraft Project Generator")]
    #[clap(author = "zeddit")]
    #[clap(version = "1.0")]
    #[clap(about = "Generates a Gradle project with all Minecraft dependencies included.", long_about = None)]
    pub struct Cli {
        pub group: String,
        pub name: String,
        #[clap(short, long, value_parser, default_value = "1.19.2")]
        pub version: String,
        #[clap(short, long, value_parser, default_value = "~/IdeaProjects")]
        pub dir: PathBuf,
        #[clap(short, long, value_parser)]
        pub kotlin: bool,
        #[clap(short, long, value_parser)]
        pub gradle_dist: Option<PathBuf>,
        #[clap(short,long, help = "Enabling this flag causes the program to overwrite the target directory when creating the project.")]
        pub overwrite: bool
    }

    impl Cli {
        pub fn change_path(&mut self) {
            let path_str = self.dir.as_path().to_str().unwrap();
            if path_str == "~/IdeaProjects" {
                self.dir = push_self(&mut push_self(&mut dirs::home_dir().unwrap(), "IdeaProjects"), &self.name)
            }
        }

    }
    fn push_self(path: & mut PathBuf, app: & str) -> PathBuf {
        path.push(app);
        let mut x = PathBuf::new();
        x.push(path);
        return x;
    }
}


fn main() {
    let mut args : Cli = Cli::parse();
    args.change_path();
    println!("{}",args.dir.as_path().to_str().unwrap());
    create_project(Box::new(args));


}

fn create_project(args: Box<Cli>) {
    let rc : Rc<Cli> = Rc::new(*args);
    let mut code = CodeGen::from(Rc::clone(&rc));
    println!("{}",code.release_ver());
    code.settings_gradle();
    let gradle = Gradle::new(Rc::clone(&rc));
    dbg!(&gradle.path);
    let prompt_res = code.prompt_empty();
    if prompt_res.is_err() {
        let x = prompt_res.err().unwrap();
        if x.0 == true {
            panic!("{}", x.1);
        }
        println!("{}","Warning: Target directory is not empty, however override flag is set, so continuing..".yellow());
        std::fs::remove_dir_all()
    }
    std::fs::create_dir(&rc.dir).unwrap();

    ()
}
