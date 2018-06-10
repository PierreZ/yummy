#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "yummy-cli")]
struct Opt {
    /// config file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
}
fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
    ]).unwrap();

    let opt = Opt::from_args();
    info!("{:?}", opt);
}
