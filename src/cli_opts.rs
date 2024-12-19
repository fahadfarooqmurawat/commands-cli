use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct CliOpts {
    #[clap(name = "args")]
    pub args: Vec<String>,  // Change this to Option<String> to make it optional
}
