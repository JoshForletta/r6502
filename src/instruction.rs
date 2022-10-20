use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    addressing_mode::{AddressingMode, AmFn, IMPLIED},
    error::InstructionError,
    R6502,
};

#[derive(Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub am: AddressingMode,
    call: fn(&mut R6502, AmFn) -> Result<(), Box<dyn Error>>,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("opcode", &format_args!("${:02X}", self.opcode))
            .field("mneumonic", &self.mnemonic)
            .finish()
    }
}

impl Instruction {
    pub fn exec(&self, cpu: &mut R6502) -> Result<(), InstructionError> {
        cpu.extra_cycles += 1;
        match (self.call)(cpu, self.am.call) {
            Ok(_) => Ok(()),
            Err(e) => Err(InstructionError {
                instruction: self.clone(),
                error: e,
            }),
        }
    }
}

#[derive(Debug)]
pub struct IllegalInstruction;

impl Display for IllegalInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal Instruction")
    }
}

impl Error for IllegalInstruction {}

fn illegal(_cpu: &mut R6502, _am: AmFn) -> Result<(), Box<dyn Error>> {
    Err(IllegalInstruction {}.into())
}

pub mod adc;
pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bit;
pub mod bmi;
pub mod bne;
pub mod bpl;
pub mod brk;
pub mod bvc;
pub mod bvs;
pub mod clc;
pub mod cld;
pub mod cli;
pub mod clv;
pub mod cmp;
pub mod cpx;
pub mod cpy;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod lsr;
pub mod nop;
pub mod ora;
pub mod pha;
pub mod php;
pub mod pla;
pub mod plp;
pub mod rol;
pub mod ror;
pub mod rti;
pub mod rts;
pub mod sbc;
pub mod sec;
pub mod sed;
pub mod sei;
pub mod sta;
pub mod stx;
pub mod sty;
pub mod tax;
pub mod tay;
pub mod tsx;
pub mod txa;
pub mod txs;
pub mod tya;

pub const INSTRUCTION_SET: &[Instruction] = &[
    brk::BRK_IMPLIED,
    ora::ORA_INDEXED_INDIRECT,
    Instruction {
        opcode: 0x02,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x03,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x04,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ora::ORA_ZERO_PAGE,
    asl::ASL_ZERO_PAGE,
    Instruction {
        opcode: 0x07,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    php::PHP_IMPLIED,
    ora::ORA_IMMEDIATE,
    asl::ASL_ACCUMULATOR,
    Instruction {
        opcode: 0x0B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x0C,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ora::ORA_ABSOLUTE,
    asl::ASL_ABSOLUTE,
    Instruction {
        opcode: 0x0F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bpl::BPL_RELATIVE,
    ora::ORA_INDIRECT_INDEXED,
    Instruction {
        opcode: 0x12,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x13,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x14,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ora::ORA_ZERO_PAGE_X,
    asl::ASL_ZERO_PAGE_X,
    Instruction {
        opcode: 0x17,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    clc::CLC_IMPLIED,
    ora::ORA_ABSOLUTE_Y,
    Instruction {
        opcode: 0x1A,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x1B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x1C,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ora::ORA_ABSOLUTE_X,
    asl::ASL_ABSOLUTE_X,
    Instruction {
        opcode: 0x1F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    jsr::JSR_ABSOLUTE,
    and::AND_INDEXED_INDIRECT,
    Instruction {
        opcode: 0x22,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x23,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bit::BIT_ZERO_PAGE,
    and::AND_ZERO_PAGE,
    rol::ROL_ZERO_PAGE,
    Instruction {
        opcode: 0x27,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    plp::PLP_IMPLIED,
    and::AND_IMMEDIATE,
    rol::ROL_ACCUMULATOR,
    Instruction {
        opcode: 0x2B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bit::BIT_ABSOLUTE,
    and::AND_ABSOLUTE,
    rol::ROL_ABSOLUTE,
    Instruction {
        opcode: 0x2F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bmi::BMI_RELATIVE,
    and::AND_INDIRECT_INDEXED,
    Instruction {
        opcode: 0x32,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x33,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x34,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    and::AND_ZERO_PAGE_X,
    rol::ROL_ZERO_PAGE_X,
    Instruction {
        opcode: 0x37,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sec::SEC_IMPLIED,
    and::AND_ABSOLUTE_Y,
    Instruction {
        opcode: 0x3A,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x3B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x3C,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    and::AND_ABSOLUTE_X,
    rol::ROL_ABSOLUTE_X,
    Instruction {
        opcode: 0x3F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    rti::RTI_IMPLIED,
    eor::EOR_INDEXED_INDIRECT,
    Instruction {
        opcode: 0x42,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x43,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x44,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    eor::EOR_ZERO_PAGE,
    lsr::LSR_ZERO_PAGE,
    Instruction {
        opcode: 0x47,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    pha::PHA_IMPLIED,
    eor::EOR_IMMEDIATE,
    lsr::LSR_ACCUMULATOR,
    Instruction {
        opcode: 0x4B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    jmp::JMP_ABSOLUTE,
    eor::EOR_ABSOLUTE,
    lsr::LSR_ABSOLUTE,
    Instruction {
        opcode: 0x4F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bvc::BVC_RELATIVE,
    eor::EOR_INDIRECT_INDEXED,
    Instruction {
        opcode: 0x52,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x53,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x54,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    eor::EOR_ZERO_PAGE_X,
    lsr::LSR_ZERO_PAGE_X,
    Instruction {
        opcode: 0x57,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cli::CLI_IMPLIED,
    eor::EOR_ABSOLUTE_Y,
    Instruction {
        opcode: 0x5A,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x5B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x5C,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    eor::EOR_ABSOLUTE_X,
    lsr::LSR_ABSOLUTE_X,
    Instruction {
        opcode: 0x5F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    rts::RTS_IMPLIED,
    adc::ADC_INDEXED_INDIRECT,
    Instruction {
        opcode: 0x62,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x63,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x64,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    adc::ADC_ZERO_PAGE,
    ror::ROR_ZERO_PAGE,
    Instruction {
        opcode: 0x67,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    pla::PLA_IMPLIED,
    adc::ADC_IMMEDIATE,
    ror::ROR_ACCUMULATOR,
    Instruction {
        opcode: 0x6B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    jmp::JMP_INDIRECT,
    adc::ADC_ABSOLUTE,
    ror::ROR_ABSOLUTE,
    Instruction {
        opcode: 0x6F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bvs::BVS_RELATIVE,
    adc::ADC_INDIRECT_INDEXED,
    Instruction {
        opcode: 0x72,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x73,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x74,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    adc::ADC_ZERO_PAGE_X,
    ror::ROR_ZERO_PAGE_X,
    Instruction {
        opcode: 0x77,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sei::SEI_IMPLIED,
    adc::ADC_ABSOLUTE_Y,
    Instruction {
        opcode: 0x7A,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x7B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x7C,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    adc::ADC_ABSOLUTE_X,
    ror::ROR_ABSOLUTE_X,
    Instruction {
        opcode: 0x7F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x80,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sta::STA_INDEXED_INDIRECT,
    Instruction {
        opcode: 0x82,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x83,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sty::STY_ZERO_PAGE,
    sta::STA_ZERO_PAGE,
    stx::STX_ZERO_PAGE,
    Instruction {
        opcode: 0x87,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    dey::DEY_IMPLIED,
    Instruction {
        opcode: 0x89,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    txa::TXA_IMPLIED,
    Instruction {
        opcode: 0x8B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sty::STY_ABSOLUTE,
    sta::STA_ABSOLUTE,
    stx::STX_ABSOLUTE,
    Instruction {
        opcode: 0x8F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bcc::BCC_RELATIVE,
    sta::STA_INDIRECT_INDEXED,
    Instruction {
        opcode: 0x92,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x93,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sty::STY_ZERO_PAGE_X,
    sta::STA_ZERO_PAGE_X,
    stx::STX_ZERO_PAGE_Y,
    Instruction {
        opcode: 0x97,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    tya::TYA_IMPLIED,
    sta::STA_ABSOLUTE_Y,
    txs::TXS_IMPLIED,
    Instruction {
        opcode: 0x9B,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x9C,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sta::STA_ABSOLUTE_X,
    Instruction {
        opcode: 0x9E,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0x9F,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ldy::LDY_IMMEDIATE,
    lda::LDA_INDEXED_INDIRECT,
    ldx::LDX_IMMEDIATE,
    Instruction {
        opcode: 0xA3,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ldy::LDY_ZERO_PAGE,
    lda::LDA_ZERO_PAGE,
    ldx::LDX_ZERO_PAGE,
    Instruction {
        opcode: 0xA7,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    tay::TAY_IMPLIED,
    lda::LDA_IMMEDIATE,
    tax::TAX_IMPLIED,
    Instruction {
        opcode: 0xAB,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ldy::LDY_ABSOLUTE,
    lda::LDA_ABSOLUTE,
    ldx::LDX_ABSOLUTE,
    Instruction {
        opcode: 0xAF,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bcs::BCS_RELATIVE,
    lda::LDA_INDIRECT_INDEXED,
    Instruction {
        opcode: 0xB2,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xB3,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ldy::LDY_ZERO_PAGE_X,
    lda::LDA_ZERO_PAGE_X,
    ldx::LDX_ZERO_PAGE_Y,
    Instruction {
        opcode: 0xB7,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    clv::CLV_IMPLIED,
    lda::LDA_ABSOLUTE_Y,
    tsx::TSX_IMPLIED,
    Instruction {
        opcode: 0xBB,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    ldy::LDY_ABSOLUTE_X,
    lda::LDA_ABSOLUTE_X,
    ldx::LDX_ABSOLUTE_Y,
    Instruction {
        opcode: 0xBF,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cpy::CPY_IMMEDIATE,
    cmp::CMP_INDEXED_INDIRECT,
    Instruction {
        opcode: 0xC2,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xC3,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cpy::CPY_ZERO_PAGE,
    cmp::CMP_ZERO_PAGE,
    dec::DEC_ZERO_PAGE,
    Instruction {
        opcode: 0xC7,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    iny::INY_IMPLIED,
    cmp::CMP_IMMEDIATE,
    dex::DEX_IMPLIED,
    Instruction {
        opcode: 0xCB,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cpy::CPY_ABSOLUTE,
    cmp::CMP_ABSOLUTE,
    dec::DEC_ABSOLUTE,
    Instruction {
        opcode: 0xCF,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    bne::BNE_RELATIVE,
    cmp::CMP_INDIRECT_INDEXED,
    Instruction {
        opcode: 0xD2,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xD3,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xD4,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cmp::CMP_ZERO_PAGE_X,
    dec::DEC_ZERO_PAGE_X,
    Instruction {
        opcode: 0xD7,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cld::CLD_IMPLIED,
    cmp::CMP_ABSOLUTE_Y,
    Instruction {
        opcode: 0xDA,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xDB,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xDC,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cmp::CMP_ABSOLUTE_X,
    dec::DEC_ABSOLUTE_X,
    Instruction {
        opcode: 0xDF,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cpx::CPX_IMMEDIATE,
    sbc::SBC_INDEXED_INDIRECT,
    Instruction {
        opcode: 0xE2,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xE3,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cpx::CPX_ZERO_PAGE,
    sbc::SBC_ZERO_PAGE,
    inc::INC_ZERO_PAGE,
    Instruction {
        opcode: 0xE7,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    inx::INX_IMPLIED,
    sbc::SBC_IMMEDIATE,
    nop::NOP_IMPLIED,
    Instruction {
        opcode: 0xEB,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    cpx::CPX_ABSOLUTE,
    sbc::SBC_ABSOLUTE,
    inc::INC_ABSOLUTE,
    Instruction {
        opcode: 0xEF,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    beq::BEQ_RELATIVE,
    sbc::SBC_INDIRECT_INDEXED,
    Instruction {
        opcode: 0xF2,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xF3,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xF4,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sbc::SBC_ZERO_PAGE_X,
    inc::INC_ZERO_PAGE_X,
    Instruction {
        opcode: 0xF7,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sed::SED_IMPLIED,
    sbc::SBC_ABSOLUTE_Y,
    Instruction {
        opcode: 0xFA,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xFB,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    Instruction {
        opcode: 0xFC,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
    sbc::SBC_ABSOLUTE_X,
    inc::INC_ABSOLUTE_X,
    Instruction {
        opcode: 0xFF,
        mnemonic: "ILLEGAL",
        am: IMPLIED,
        call: illegal,
    },
];
