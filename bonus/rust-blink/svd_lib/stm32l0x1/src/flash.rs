#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    #[doc = "0x04 - Program/erase control register"]
    pub pecr: crate::Reg<pecr::PECR_SPEC>,
    #[doc = "0x08 - Power down key register"]
    pub pdkeyr: crate::Reg<pdkeyr::PDKEYR_SPEC>,
    #[doc = "0x0c - Program/erase key register"]
    pub pekeyr: crate::Reg<pekeyr::PEKEYR_SPEC>,
    #[doc = "0x10 - Program memory key register"]
    pub prgkeyr: crate::Reg<prgkeyr::PRGKEYR_SPEC>,
    #[doc = "0x14 - Option byte key register"]
    pub optkeyr: crate::Reg<optkeyr::OPTKEYR_SPEC>,
    #[doc = "0x18 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x1c - Option byte register"]
    pub obr: crate::Reg<obr::OBR_SPEC>,
    #[doc = "0x20 - Write protection register"]
    pub wrpr: crate::Reg<wrpr::WRPR_SPEC>,
}
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "PECR register accessor: an alias for `Reg<PECR_SPEC>`"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "Program/erase control register"]
pub mod pecr;
#[doc = "PDKEYR register accessor: an alias for `Reg<PDKEYR_SPEC>`"]
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYR_SPEC>;
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "PEKEYR register accessor: an alias for `Reg<PEKEYR_SPEC>`"]
pub type PEKEYR = crate::Reg<pekeyr::PEKEYR_SPEC>;
#[doc = "Program/erase key register"]
pub mod pekeyr;
#[doc = "PRGKEYR register accessor: an alias for `Reg<PRGKEYR_SPEC>`"]
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYR_SPEC>;
#[doc = "Program memory key register"]
pub mod prgkeyr;
#[doc = "OPTKEYR register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "OBR register accessor: an alias for `Reg<OBR_SPEC>`"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR register accessor: an alias for `Reg<WRPR_SPEC>`"]
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
#[doc = "Write protection register"]
pub mod wrpr;
