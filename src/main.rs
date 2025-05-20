use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::Result;
use jsol_parse::{RawJsolFile, RawOperation};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
	EnvFilter,
	fmt::{self, format::FmtSpan},
	prelude::*,
};

fn main() -> Result<()> {
	_ = fs::remove_dir_all("./out");

	fs::create_dir_all("./out")?;
	install_tracing();
	color_eyre::install()?;

	let args = Args::parse();

	let raw_data = fs::File::open(&args.file)?;

	let mut raw_program = serde_json::from_reader::<_, RawJsolFile>(raw_data)?;

	{
		let Some(operations) = raw_program.operations_mut() else {
			panic!("blah");
		};

		operations.clear();

		operations.push(RawOperation::Nop);
	}

	fs::write(&args.file, serde_json::to_string_pretty(&raw_program)?)?;

	Ok(())
}

fn install_tracing() {
	fs::create_dir_all("./out").unwrap();

	let log_file = fs::OpenOptions::new()
		.create(true)
		.truncate(true)
		.write(true)
		.open("./out/output.log")
		.expect("failed to open file");

	let json_log_file = fs::OpenOptions::new()
		.create(true)
		.truncate(true)
		.write(true)
		.open("./out/output.json")
		.expect("failed to open file");

	let file_layer = fmt::layer().with_ansi(false).with_writer(log_file);

	let filter_layer = EnvFilter::new("info");
	let fmt_layer = fmt::layer().with_target(false).with_filter(filter_layer);

	let json_file_layer = fmt::layer()
		.with_ansi(false)
		.json()
		.flatten_event(true)
		.with_span_events(FmtSpan::FULL)
		.with_writer(json_log_file);

	tracing_subscriber::registry()
		.with(json_file_layer)
		.with(file_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}

#[derive(Debug, Parser)]
struct Args {
	pub file: PathBuf,
	#[arg(short, long)]
	pub optimize: bool,
}
