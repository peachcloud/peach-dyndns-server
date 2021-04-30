use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "peach-dyndns-host",
    rename_all = "kebab-case",
    long_about = "\nTODO",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub struct CliArgs {
    #[structopt(flatten)]
    log: clap_log_flag::Log,
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

pub fn args() -> Result<CliArgs, Box<dyn std::error::Error>> {
    let args = CliArgs::from_args();

    args.log.log_all(Some(args.verbose.log_level()))?;

    Ok(args)
}
