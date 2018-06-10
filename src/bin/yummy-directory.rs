#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate yummy;

use simplelog::*;
use structopt::StructOpt;
use yummy::config;
use std::error::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "yummy-directory")]
struct Opt {
    /// config file
    #[structopt(short = "c", long = "config")]
    config: String,
}

fn main() -> Result<(), Box<Error>> {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
    ]).unwrap();

    let opt = Opt::from_args();
    let setting = config::Settings::from(opt.config)?;
    
    info!("Starting directory with {:?}", setting);
    
    Ok(())
}
