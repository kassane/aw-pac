#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - SPI Global Control Register"]
    pub spi_gcr: SPI_GCR,
    #[doc = "0x08 - SPI Transfer Control Register"]
    pub spi_tcr: SPI_TCR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - SPI Interrupt Control Register"]
    pub spi_ier: SPI_IER,
    #[doc = "0x14 - SPI Interrupt Status Register"]
    pub spi_isr: SPI_ISR,
    #[doc = "0x18 - SPI FIFO Control Register"]
    pub spi_fcr: SPI_FCR,
    #[doc = "0x1c - SPI FIFO Status Register"]
    pub spi_fsr: SPI_FSR,
    #[doc = "0x20 - SPI Wait Clock Register"]
    pub spi_wcr: SPI_WCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - SPI Sample Delay Control Register"]
    pub spi_samp_dl: SPI_SAMP_DL,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - SPI Master Burst Counter Register"]
    pub spi_mbc: SPI_MBC,
    #[doc = "0x34 - SPI Master Transmit Counter Register"]
    pub spi_mtc: SPI_MTC,
    #[doc = "0x38 - SPI Master Burst Control Register"]
    pub spi_bcc: SPI_BCC,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - SPI Bit-Aligned Transfer Configure Register"]
    pub spi_batc: SPI_BATC,
    #[doc = "0x44 - SPI Bit-Aligned Clock Configuration Register"]
    pub spi_ba_ccr: SPI_BA_CCR,
    #[doc = "0x48 - SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits"]
    pub spi_tbr: SPI_TBR,
    #[doc = "0x4c - SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits"]
    pub spi_rbr: SPI_RBR,
    _reserved15: [u8; 0x38],
    #[doc = "0x88 - SPI Normal DMA Mode Control Register"]
    pub spi_ndma_mode_ctl: SPI_NDMA_MODE_CTL,
    _reserved16: [u8; 0x74],
    #[doc = "0x100 - DBI Control Register 0"]
    pub dbi_ctl_0: DBI_CTL_0,
    #[doc = "0x104 - DBI Control Register 1"]
    pub dbi_ctl_1: DBI_CTL_1,
    #[doc = "0x108 - DBI Control Register 2"]
    pub dbi_ctl_2: DBI_CTL_2,
    #[doc = "0x10c - DBI Timer Control Register"]
    pub dbi_timer: DBI_TIMER,
    #[doc = "0x110 - DBI Video Size Configuration Register"]
    pub dbi_video_szie: DBI_VIDEO_SZIE,
    _reserved21: [u8; 0x0c],
    #[doc = "0x120 - DBI Interrupt Register"]
    pub dbi_int: DBI_INT,
    #[doc = "0x124 - DBI BEBUG 0 Register"]
    pub dbi_debug_0: DBI_DEBUG_0,
    #[doc = "0x128 - DBI BEBUG 1 Register"]
    pub dbi_debug_1: DBI_DEBUG_1,
    _reserved24: [u8; 0xd4],
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data"]
    pub spi_txd: SPI_TXD,
    _reserved25: [u8; 0xfc],
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data"]
    pub spi_rxd: SPI_RXD,
}
#[doc = "spi_gcr (rw) register accessor: an alias for `Reg<SPI_GCR_SPEC>`"]
pub type SPI_GCR = crate::Reg<spi_gcr::SPI_GCR_SPEC>;
#[doc = "SPI Global Control Register"]
pub mod spi_gcr;
#[doc = "spi_tcr (rw) register accessor: an alias for `Reg<SPI_TCR_SPEC>`"]
pub type SPI_TCR = crate::Reg<spi_tcr::SPI_TCR_SPEC>;
#[doc = "SPI Transfer Control Register"]
pub mod spi_tcr;
#[doc = "spi_ier (rw) register accessor: an alias for `Reg<SPI_IER_SPEC>`"]
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
#[doc = "SPI Interrupt Control Register"]
pub mod spi_ier;
#[doc = "spi_isr (rw) register accessor: an alias for `Reg<SPI_ISR_SPEC>`"]
pub type SPI_ISR = crate::Reg<spi_isr::SPI_ISR_SPEC>;
#[doc = "SPI Interrupt Status Register"]
pub mod spi_isr;
#[doc = "spi_fcr (rw) register accessor: an alias for `Reg<SPI_FCR_SPEC>`"]
pub type SPI_FCR = crate::Reg<spi_fcr::SPI_FCR_SPEC>;
#[doc = "SPI FIFO Control Register"]
pub mod spi_fcr;
#[doc = "spi_fsr (r) register accessor: an alias for `Reg<SPI_FSR_SPEC>`"]
pub type SPI_FSR = crate::Reg<spi_fsr::SPI_FSR_SPEC>;
#[doc = "SPI FIFO Status Register"]
pub mod spi_fsr;
#[doc = "spi_wcr (rw) register accessor: an alias for `Reg<SPI_WCR_SPEC>`"]
pub type SPI_WCR = crate::Reg<spi_wcr::SPI_WCR_SPEC>;
#[doc = "SPI Wait Clock Register"]
pub mod spi_wcr;
#[doc = "spi_samp_dl (rw) register accessor: an alias for `Reg<SPI_SAMP_DL_SPEC>`"]
pub type SPI_SAMP_DL = crate::Reg<spi_samp_dl::SPI_SAMP_DL_SPEC>;
#[doc = "SPI Sample Delay Control Register"]
pub mod spi_samp_dl;
#[doc = "spi_mbc (rw) register accessor: an alias for `Reg<SPI_MBC_SPEC>`"]
pub type SPI_MBC = crate::Reg<spi_mbc::SPI_MBC_SPEC>;
#[doc = "SPI Master Burst Counter Register"]
pub mod spi_mbc;
#[doc = "spi_mtc (rw) register accessor: an alias for `Reg<SPI_MTC_SPEC>`"]
pub type SPI_MTC = crate::Reg<spi_mtc::SPI_MTC_SPEC>;
#[doc = "SPI Master Transmit Counter Register"]
pub mod spi_mtc;
#[doc = "spi_bcc (rw) register accessor: an alias for `Reg<SPI_BCC_SPEC>`"]
pub type SPI_BCC = crate::Reg<spi_bcc::SPI_BCC_SPEC>;
#[doc = "SPI Master Burst Control Register"]
pub mod spi_bcc;
#[doc = "spi_batc (rw) register accessor: an alias for `Reg<SPI_BATC_SPEC>`"]
pub type SPI_BATC = crate::Reg<spi_batc::SPI_BATC_SPEC>;
#[doc = "SPI Bit-Aligned Transfer Configure Register"]
pub mod spi_batc;
#[doc = "spi_ba_ccr (rw) register accessor: an alias for `Reg<SPI_BA_CCR_SPEC>`"]
pub type SPI_BA_CCR = crate::Reg<spi_ba_ccr::SPI_BA_CCR_SPEC>;
#[doc = "SPI Bit-Aligned Clock Configuration Register"]
pub mod spi_ba_ccr;
#[doc = "spi_tbr (rw) register accessor: an alias for `Reg<SPI_TBR_SPEC>`"]
pub type SPI_TBR = crate::Reg<spi_tbr::SPI_TBR_SPEC>;
#[doc = "SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits"]
pub mod spi_tbr;
#[doc = "spi_rbr (rw) register accessor: an alias for `Reg<SPI_RBR_SPEC>`"]
pub type SPI_RBR = crate::Reg<spi_rbr::SPI_RBR_SPEC>;
#[doc = "SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits"]
pub mod spi_rbr;
#[doc = "spi_ndma_mode_ctl (rw) register accessor: an alias for `Reg<SPI_NDMA_MODE_CTL_SPEC>`"]
pub type SPI_NDMA_MODE_CTL = crate::Reg<spi_ndma_mode_ctl::SPI_NDMA_MODE_CTL_SPEC>;
#[doc = "SPI Normal DMA Mode Control Register"]
pub mod spi_ndma_mode_ctl;
#[doc = "dbi_ctl_0 (rw) register accessor: an alias for `Reg<DBI_CTL_0_SPEC>`"]
pub type DBI_CTL_0 = crate::Reg<dbi_ctl_0::DBI_CTL_0_SPEC>;
#[doc = "DBI Control Register 0"]
pub mod dbi_ctl_0;
#[doc = "dbi_ctl_1 (rw) register accessor: an alias for `Reg<DBI_CTL_1_SPEC>`"]
pub type DBI_CTL_1 = crate::Reg<dbi_ctl_1::DBI_CTL_1_SPEC>;
#[doc = "DBI Control Register 1"]
pub mod dbi_ctl_1;
#[doc = "dbi_ctl_2 (rw) register accessor: an alias for `Reg<DBI_CTL_2_SPEC>`"]
pub type DBI_CTL_2 = crate::Reg<dbi_ctl_2::DBI_CTL_2_SPEC>;
#[doc = "DBI Control Register 2"]
pub mod dbi_ctl_2;
#[doc = "dbi_timer (rw) register accessor: an alias for `Reg<DBI_TIMER_SPEC>`"]
pub type DBI_TIMER = crate::Reg<dbi_timer::DBI_TIMER_SPEC>;
#[doc = "DBI Timer Control Register"]
pub mod dbi_timer;
#[doc = "dbi_video_szie (rw) register accessor: an alias for `Reg<DBI_VIDEO_SZIE_SPEC>`"]
pub type DBI_VIDEO_SZIE = crate::Reg<dbi_video_szie::DBI_VIDEO_SZIE_SPEC>;
#[doc = "DBI Video Size Configuration Register"]
pub mod dbi_video_szie;
#[doc = "dbi_int (rw) register accessor: an alias for `Reg<DBI_INT_SPEC>`"]
pub type DBI_INT = crate::Reg<dbi_int::DBI_INT_SPEC>;
#[doc = "DBI Interrupt Register"]
pub mod dbi_int;
#[doc = "dbi_debug_0 (r) register accessor: an alias for `Reg<DBI_DEBUG_0_SPEC>`"]
pub type DBI_DEBUG_0 = crate::Reg<dbi_debug_0::DBI_DEBUG_0_SPEC>;
#[doc = "DBI BEBUG 0 Register"]
pub mod dbi_debug_0;
#[doc = "dbi_debug_1 (r) register accessor: an alias for `Reg<DBI_DEBUG_1_SPEC>`"]
pub type DBI_DEBUG_1 = crate::Reg<dbi_debug_1::DBI_DEBUG_1_SPEC>;
#[doc = "DBI BEBUG 1 Register"]
pub mod dbi_debug_1;
#[doc = "spi_txd (rw) register accessor: an alias for `Reg<SPI_TXD_SPEC>`"]
pub type SPI_TXD = crate::Reg<spi_txd::SPI_TXD_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data"]
pub mod spi_txd;
#[doc = "spi_rxd (rw) register accessor: an alias for `Reg<SPI_RXD_SPEC>`"]
pub type SPI_RXD = crate::Reg<spi_rxd::SPI_RXD_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data"]
pub mod spi_rxd;
