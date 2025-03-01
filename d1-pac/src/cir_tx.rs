#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CIR Transmit Global Register"]
    pub cir_tglr: CIR_TGLR,
    #[doc = "0x04 - CIR Transmit Modulation Control Register"]
    pub cir_tmcr: CIR_TMCR,
    #[doc = "0x08 - CIR Transmit Control Register"]
    pub cir_tcr: CIR_TCR,
    #[doc = "0x0c - CIR Transmit Idle Duration Threshold High Bit Register"]
    pub cir_idc_h: CIR_IDC_H,
    #[doc = "0x10 - CIR Transmit Idle Duration Threshold Low Bit Register"]
    pub cir_idc_l: CIR_IDC_L,
    #[doc = "0x14 - CIR Transmit Idle Counter High Bit Register"]
    pub cir_ticr_h: CIR_TICR_H,
    #[doc = "0x18 - CIR Transmit Idle Counter Low Bit Register"]
    pub cir_ticr_l: CIR_TICR_L,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - CIR TX FIFO Empty Level Register"]
    pub cir_tel: CIR_TEL,
    #[doc = "0x24 - CIR Transmit Interrupt Control Register"]
    pub cir_txint: CIR_TXINT,
    #[doc = "0x28 - CIR Transmit FIFO Available Counter Register"]
    pub cir_tac: CIR_TAC,
    #[doc = "0x2c - CIR Transmit Status Register"]
    pub cir_txsta: CIR_TXSTA,
    #[doc = "0x30 - CIR Transmit Threshold Register"]
    pub cir_txt: CIR_TXT,
    #[doc = "0x34 - CIR DMA Control Register"]
    pub cir_dma_ctl: CIR_DMA_CTL,
    _reserved13: [u8; 0x48],
    #[doc = "0x80 - CIR Transmit FIFO Data Register"]
    pub cir_txfifo: CIR_TXFIFO,
}
#[doc = "cir_tglr (rw) register accessor: an alias for `Reg<CIR_TGLR_SPEC>`"]
pub type CIR_TGLR = crate::Reg<cir_tglr::CIR_TGLR_SPEC>;
#[doc = "CIR Transmit Global Register"]
pub mod cir_tglr;
#[doc = "cir_tmcr (rw) register accessor: an alias for `Reg<CIR_TMCR_SPEC>`"]
pub type CIR_TMCR = crate::Reg<cir_tmcr::CIR_TMCR_SPEC>;
#[doc = "CIR Transmit Modulation Control Register"]
pub mod cir_tmcr;
#[doc = "cir_tcr (rw) register accessor: an alias for `Reg<CIR_TCR_SPEC>`"]
pub type CIR_TCR = crate::Reg<cir_tcr::CIR_TCR_SPEC>;
#[doc = "CIR Transmit Control Register"]
pub mod cir_tcr;
#[doc = "cir_idc_h (rw) register accessor: an alias for `Reg<CIR_IDC_H_SPEC>`"]
pub type CIR_IDC_H = crate::Reg<cir_idc_h::CIR_IDC_H_SPEC>;
#[doc = "CIR Transmit Idle Duration Threshold High Bit Register"]
pub mod cir_idc_h;
#[doc = "cir_idc_l (rw) register accessor: an alias for `Reg<CIR_IDC_L_SPEC>`"]
pub type CIR_IDC_L = crate::Reg<cir_idc_l::CIR_IDC_L_SPEC>;
#[doc = "CIR Transmit Idle Duration Threshold Low Bit Register"]
pub mod cir_idc_l;
#[doc = "cir_ticr_h (rw) register accessor: an alias for `Reg<CIR_TICR_H_SPEC>`"]
pub type CIR_TICR_H = crate::Reg<cir_ticr_h::CIR_TICR_H_SPEC>;
#[doc = "CIR Transmit Idle Counter High Bit Register"]
pub mod cir_ticr_h;
#[doc = "cir_ticr_l (rw) register accessor: an alias for `Reg<CIR_TICR_L_SPEC>`"]
pub type CIR_TICR_L = crate::Reg<cir_ticr_l::CIR_TICR_L_SPEC>;
#[doc = "CIR Transmit Idle Counter Low Bit Register"]
pub mod cir_ticr_l;
#[doc = "cir_tel (rw) register accessor: an alias for `Reg<CIR_TEL_SPEC>`"]
pub type CIR_TEL = crate::Reg<cir_tel::CIR_TEL_SPEC>;
#[doc = "CIR TX FIFO Empty Level Register"]
pub mod cir_tel;
#[doc = "cir_txint (rw) register accessor: an alias for `Reg<CIR_TXINT_SPEC>`"]
pub type CIR_TXINT = crate::Reg<cir_txint::CIR_TXINT_SPEC>;
#[doc = "CIR Transmit Interrupt Control Register"]
pub mod cir_txint;
#[doc = "cir_tac (rw) register accessor: an alias for `Reg<CIR_TAC_SPEC>`"]
pub type CIR_TAC = crate::Reg<cir_tac::CIR_TAC_SPEC>;
#[doc = "CIR Transmit FIFO Available Counter Register"]
pub mod cir_tac;
#[doc = "cir_txsta (rw) register accessor: an alias for `Reg<CIR_TXSTA_SPEC>`"]
pub type CIR_TXSTA = crate::Reg<cir_txsta::CIR_TXSTA_SPEC>;
#[doc = "CIR Transmit Status Register"]
pub mod cir_txsta;
#[doc = "cir_txt (rw) register accessor: an alias for `Reg<CIR_TXT_SPEC>`"]
pub type CIR_TXT = crate::Reg<cir_txt::CIR_TXT_SPEC>;
#[doc = "CIR Transmit Threshold Register"]
pub mod cir_txt;
#[doc = "cir_dma_ctl (rw) register accessor: an alias for `Reg<CIR_DMA_CTL_SPEC>`"]
pub type CIR_DMA_CTL = crate::Reg<cir_dma_ctl::CIR_DMA_CTL_SPEC>;
#[doc = "CIR DMA Control Register"]
pub mod cir_dma_ctl;
#[doc = "cir_txfifo (rw) register accessor: an alias for `Reg<CIR_TXFIFO_SPEC>`"]
pub type CIR_TXFIFO = crate::Reg<cir_txfifo::CIR_TXFIFO_SPEC>;
#[doc = "CIR Transmit FIFO Data Register"]
pub mod cir_txfifo;
