use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(ops) => {
            let output = if let Some(output) = ops.output {
                output
            } else {
                format!("output.{}", ops.format)
            };
            process_csv(&ops.input, output, ops.format)?;
        }
    }
    Ok(())
}
