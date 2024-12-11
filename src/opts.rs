use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    #[clap(name = "args")]
    pub args: Vec<String>,  // Change this to Option<String> to make it optional
}
