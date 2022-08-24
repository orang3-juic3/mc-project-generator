mod files;
mod gradlecreator;

use clap::Parser;
use std::path::PathBuf;
use std::path::Path;
use crate::cli::Cli;
use crate::files::CodeGen;
use crate::gradlecreator::Gradle;
use std::rc::Rc;

mod cli {
    use clap::Parser;
    use std::path::PathBuf;
    use std::path::Path;
    #[derive(Parser)]
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
        pub gradle_dist: Option<PathBuf>
    }

    impl Cli {
        pub fn change_path(&mut self) {
            let path_str = self.dir.as_path().to_str().unwrap();
            if path_str == "~/IdeaProjects" {
                self.dir = push_self(&mut dirs::home_dir().unwrap(), "IdeaProjects")
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
    crate::gradlecreator::Gra

}


fn create_project(args: Box<Cli>) {
    let rc : Rc<Cli> = Rc::new(*args);
    let mut code = CodeGen::from(Rc::clone(&rc));
    println!("{}",code.release_ver());
    code.settings_gradle();
}
