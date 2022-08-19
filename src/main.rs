mod files;

use clap::Parser;
use std::path::PathBuf;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    group: String,
    name: String,
    #[clap(default_value = "~/IdeaProjects")]
    dir: PathBuf,
    kotlin: bool,


}

impl Cli {
    fn change_defaults(self) -> Self {
        let path_str = self.dir.as_path().to_str().unwrap();
        Self {
            group: self.group,
            dir: if path_str == "~/IdeaProjects" { push_self(&mut dirs::home_dir().unwrap(), "IdeaProjects")}
            else {self.dir},
            kotlin: false,
            name: self.name
        }
    }
}
fn main() {
    let args : Cli = Cli::change_defaults(Cli::parse());
    println!("{}",args.dir.as_path().to_str().unwrap())
}

fn push_self(path: & mut PathBuf, app: & str) -> PathBuf {
    path.push(app);
    let mut x = PathBuf::new();
    x.push(path);
    return x;
}
