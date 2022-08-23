mod files;

use clap::Parser;
use std::path::PathBuf;
use std::path::Path;
use crate::cli::Cli;
use crate::files::CodeGen;

mod cli {
    use clap::Parser;
    use std::path::PathBuf;
    use std::path::Path;
    #[derive(Parser)]
    pub struct Cli {
        pub group: String,
        pub name: String,
        #[clap(default_value = "1.19.2")]
        pub version: String,
        #[clap(default_value = "~/IdeaProjects")]
        pub dir: PathBuf,
        pub kotlin: bool,
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
    create_project(args);
}


fn create_project(args: Cli) {
    let mut code = CodeGen::from(Box::new(args));
    code.release_ver();
}
