use crate::certificate;
use crate::Error;
use clap::ArgMatches;

pub fn run(clap_matches: &ArgMatches) -> Result<(), Error> {
    certificate::run(clap_matches, true)
}
