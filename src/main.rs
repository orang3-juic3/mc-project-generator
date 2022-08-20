mod files;

use clap::Parser;
use std::path::PathBuf;
use std::path::Path;
use crate::cli::Cli;

mod cli {
    use clap::Parser;
    use std::path::PathBuf;
    use std::path::Path;
    #[derive(Parser)]
    pub struct Cli {
        pub group: String,
        pub name: String,
        #[clap(default_value = "~/IdeaProjects")]
        pub dir: PathBuf,
        pub kotlin: bool,
    }

    impl Cli {
        pub fn change_defaults(self) -> Self {
            let path_str = self.dir.as_path().to_str().unwrap();
            Self {
                dir: if path_str == "~/IdeaProjects" { push_self(&mut dirs::home_dir().unwrap(), "IdeaProjects")}
                else {self.dir},
                ..self
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
    let args : Cli = Cli::change_defaults(Cli::parse());
    println!("{}",args.dir.as_path().to_str().unwrap())
}


fn create_project() {

}
