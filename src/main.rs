/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_emu::{
	memory::SimpleImage,
	state::State, target::Target,
};

use clap::{ Arg, Command };
use std::fs::File;
use simplelog::{
	Config as LogConfig,
	LevelFilter,
	WriteLogger,
};

use std::sync::{ Arc, Mutex };

fn main() {
	let log_file = File::create("log.txt").expect("Failed to create log file");
	let matches = Command::new("as")
		.arg(Arg::new("input").required(true))
		.get_matches();

	WriteLogger::init(LevelFilter::Debug, LogConfig::default(), log_file).expect("Failed to init logger");
	
	let mut executable = File::open(&matches.get_one::<String>("input").unwrap()).expect("Failed to open binary");
	let mut s = State::new(Some(Arc::new(Mutex::new(SimpleImage::load(&mut executable)))), Target::new());

	loop {
		s.execute();
		println!("{}", s);
	}
}
