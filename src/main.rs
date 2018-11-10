extern crate failure;
#[macro_use]
extern crate log;
extern crate resolution;
extern crate stderrlog;
extern crate structopt;

use failure::Fallible;
use resolution::{OutputMode, KB};
use std::{fs::File, io::stdout, path::PathBuf, process::exit};
use structopt::StructOpt;

fn main() {
    let opts = Options::from_args();
    if let Err(err) = run(opts) {
        let mut first = true;
        let num_errs = err.iter_chain().count();
        if num_errs <= 1 {
            error!("{}", err);
        } else {
            for cause in err.iter_chain() {
                if first {
                    first = false;
                    error!("           {}", cause);
                } else {
                    error!("caused by: {}", cause);
                }
            }
        }
        let bt = err.backtrace().to_string();
        if bt != "" {
            error!("{}", bt);
        }
        exit(-1);
    }
}

fn run(opts: Options) -> Fallible<()> {
    opts.start_logger()?;

    let kb = KB::from_file(opts.path)?;
    info!("Loaded knowledge base.");
    let consistent = kb.forward_chain();

    if consistent {
        warn!("No contradiction could be found; the goal appears to be false.");
        exit(1);
    }

    let n = match opts.output_path {
        Some(path) => {
            let f = File::create(path)?;
            opts.output_mode.render_contradicted_kb(&kb, f)?
        }
        None => opts.output_mode.render_contradicted_kb(&kb, stdout())?,
    };
    debug!("Outputted {} sequents.", n);

    Ok(())
}

#[derive(StructOpt)]
#[structopt(raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
struct Options {
    /// Turns off message output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increases the verbosity. Default verbosity is warnings and higher to syslog, info and
    /// higher to the console.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// The file to read.
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    /// The output file to write to.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output_path: Option<PathBuf>,

    /// The output mode to use.
    #[structopt(short = "O", long = "output-mode", default_value = "ascii")]
    pub output_mode: OutputMode,
}

impl Options {
    /// Sets up logging as specified by the `-q` and `-v` flags.
    fn start_logger(&self) -> Fallible<()> {
        stderrlog::new()
            .verbosity(self.verbose)
            .quiet(self.quiet)
            .init()
            .map_err(From::from)
    }
}
