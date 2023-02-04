use bibe_emu::state::State;
use bibe_instr::{
	Instruction,
	Register,
	BinOp,
	rrr,
	rri,
};

fn main() {
	let mut s = State::new();
	
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
			src1: Register::r1(),
			src2: Register::r2(),
			shift: rrr::Shift {
				kind: rrr::ShiftKind::Shl,
				shift: 0
			}
		}),
		Instruction::Rrr(rrr::Instruction {
			op: BinOp::Sub,
			dest: Register::r4(),
			src1: Register::r2(),
			src2: Register::r1(),
			shift: rrr::Shift {
				kind: rrr::ShiftKind::Shl,
				shift: 0
			}
		}),
	]);

	println!("{:?}", s);
}
