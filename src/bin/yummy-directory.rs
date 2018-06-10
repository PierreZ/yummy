#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate simplelog;

extern crate yummy;

use simplelog::*;
use std::path::PathBuf;
use structopt::StructOpt;
use yummy::config;

#[derive(StructOpt, Debug)]
#[structopt(name = "yummy-directory")]
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
    info!("Starting directory with {:?}", opt);
    config::open(opt.config);
}
