#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    pub firewall_cssa: crate::Reg<firewall_cssa::FIREWALL_CSSA_SPEC>,
    #[doc = "0x04 - Code segment length"]
    pub firewall_csl: crate::Reg<firewall_csl::FIREWALL_CSL_SPEC>,
    #[doc = "0x08 - Non-volatile data segment start address"]
    pub firewall_nvdssa: crate::Reg<firewall_nvdssa::FIREWALL_NVDSSA_SPEC>,
    #[doc = "0x0c - Non-volatile data segment length"]
    pub firewall_nvdsl: crate::Reg<firewall_nvdsl::FIREWALL_NVDSL_SPEC>,
    #[doc = "0x10 - Volatile data segment start address"]
    pub firewall_vdssa: crate::Reg<firewall_vdssa::FIREWALL_VDSSA_SPEC>,
    #[doc = "0x14 - Volatile data segment length"]
    pub firewall_vdsl: crate::Reg<firewall_vdsl::FIREWALL_VDSL_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Configuration register"]
    pub firewall_cr: crate::Reg<firewall_cr::FIREWALL_CR_SPEC>,
}
#[doc = "FIREWALL_CSSA register accessor: an alias for `Reg<FIREWALL_CSSA_SPEC>`"]
pub type FIREWALL_CSSA = crate::Reg<firewall_cssa::FIREWALL_CSSA_SPEC>;
#[doc = "Code segment start address"]
pub mod firewall_cssa;
#[doc = "FIREWALL_CSL register accessor: an alias for `Reg<FIREWALL_CSL_SPEC>`"]
pub type FIREWALL_CSL = crate::Reg<firewall_csl::FIREWALL_CSL_SPEC>;
#[doc = "Code segment length"]
pub mod firewall_csl;
#[doc = "FIREWALL_NVDSSA register accessor: an alias for `Reg<FIREWALL_NVDSSA_SPEC>`"]
pub type FIREWALL_NVDSSA = crate::Reg<firewall_nvdssa::FIREWALL_NVDSSA_SPEC>;
#[doc = "Non-volatile data segment start address"]
pub mod firewall_nvdssa;
#[doc = "FIREWALL_NVDSL register accessor: an alias for `Reg<FIREWALL_NVDSL_SPEC>`"]
pub type FIREWALL_NVDSL = crate::Reg<firewall_nvdsl::FIREWALL_NVDSL_SPEC>;
#[doc = "Non-volatile data segment length"]
pub mod firewall_nvdsl;
#[doc = "FIREWALL_VDSSA register accessor: an alias for `Reg<FIREWALL_VDSSA_SPEC>`"]
pub type FIREWALL_VDSSA = crate::Reg<firewall_vdssa::FIREWALL_VDSSA_SPEC>;
#[doc = "Volatile data segment start address"]
pub mod firewall_vdssa;
#[doc = "FIREWALL_VDSL register accessor: an alias for `Reg<FIREWALL_VDSL_SPEC>`"]
pub type FIREWALL_VDSL = crate::Reg<firewall_vdsl::FIREWALL_VDSL_SPEC>;
#[doc = "Volatile data segment length"]
pub mod firewall_vdsl;
#[doc = "FIREWALL_CR register accessor: an alias for `Reg<FIREWALL_CR_SPEC>`"]
pub type FIREWALL_CR = crate::Reg<firewall_cr::FIREWALL_CR_SPEC>;
#[doc = "Configuration register"]
pub mod firewall_cr;
