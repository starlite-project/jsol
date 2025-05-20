use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::Result;
use jsol_ir::LinkContext;
use jsol_parse::{RawConstant, RawJsolModule};
use jsol_value::Value;
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

	let mut raw_program = serde_json::from_reader::<_, RawJsolModule>(raw_data)?;

	{
		let mut cloned = raw_program.clone();

		modify_script(&mut cloned);

		if cloned != raw_program {
			fs::write(&args.file, serde_json::to_string_pretty(&cloned)?)?;

			raw_program = cloned;
		}
	}

	let mut linker = LinkContext::new();

	linker.resolve_module(raw_program);

	dbg!(linker);

	Ok(())
}

fn install_tracing() {
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

#[allow(unused)]
fn modify_script(s: &mut RawJsolModule) {
	let constants = s.constants_mut();

	constants.clear();

	constants.push(RawConstant::new("DEBUG".to_owned(), Value::Bool(true)));
}

#[derive(Debug, Parser)]
struct Args {
	pub file: PathBuf,
	#[arg(short, long)]
	pub optimize: bool,
}
