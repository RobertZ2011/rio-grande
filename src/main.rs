use bibe_emu::state::State;
use bibe_instr::{
	Instruction,
	Register,
	BinOp,
	rrr,
	rri,
};

use std::fs::File;
use simplelog::{
	Config as LogConfig,
	LevelFilter,
	WriteLogger,
};

fn main() {
	let log_file = File::create("log.txt").expect("Failed to create log file");
	let mut s = State::new();

	WriteLogger::init(LevelFilter::Debug, LogConfig::default(), log_file).expect("Failed to init logger");
	
	s.execute(&[
		Instruction::Rri(rri::Instruction {
			op: BinOp::Add,
			cond: rri::Condition::Al,
			dest: Register::r1(),
			src: Register::r0(),
			imm: 2
		}),
		Instruction::Rri(rri::Instruction {
			op: BinOp::Add,
			cond: rri::Condition::Al,
			dest: Register::r2(),
			src: Register::r0(),
			imm: 3
		}),
		Instruction::Rrr(rrr::Instruction {
			op: BinOp::Add,
			dest: Register::r3(),
			lhs: Register::r1(),
			rhs: Register::r2(),
			shift: rrr::Shift {
				kind: rrr::ShiftKind::Shl,
				shift: 0
			}
		}),
		Instruction::Rri(rri::Instruction {
			op: BinOp::Cmp,
			cond: rri::Condition::Al,
			dest: Register::r0(),
			src: Register::r3(),
			imm: 5,
		}),
		Instruction::Rri(rri::Instruction {
			op: BinOp::Add,
			cond: rri::Condition::Eq,
			dest: Register::r4(),
			src: Register::r0(),
			imm: 1,
		}),
	]);

	println!("{:?}", s);
}
