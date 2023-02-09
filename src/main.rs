use bibe_emu::{
	memory::image::Image,
	state::State,
};
use bibe_instr;
use bibe_asm_proc::asm;
use clap::{ Arg, Command };
use std::fs::File;
use simplelog::{
	Config as LogConfig,
	LevelFilter,
	WriteLogger,
};

fn main() {
	let log_file = File::create("log.txt").expect("Failed to create log file");
	let matches = Command::new("as")
		.arg(Arg::new("input").required(true))
		.get_matches();

	WriteLogger::init(LevelFilter::Debug, LogConfig::default(), log_file).expect("Failed to init logger");
	
	let mut image = File::open(&matches.get_one::<String>("input").unwrap()).expect("Failed to open binary");
	let mut s = State::new(Box::new(Image::load(&mut image)));

	loop {
		s.execute();
		println!("{:?}", s);
	}
}
