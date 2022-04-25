#[doc = "Register `CICR` reader"]
pub struct R(crate::R<CICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSSHSEC` reader - Clock Security System Interrupt clear"]
pub struct CSSHSEC_R(crate::FieldReader<bool, bool>);
impl CSSHSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSHSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSHSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSLSEC` reader - LSE Clock Security System Interrupt clear"]
pub struct CSSLSEC_R(crate::FieldReader<bool, bool>);
impl CSSLSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSLSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRDYC` reader - MSI ready Interrupt clear"]
pub struct MSIRDYC_R(crate::FieldReader<bool, bool>);
impl MSIRDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSIRDYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIRDYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLRDYC` reader - PLL ready Interrupt clear"]
pub struct PLLRDYC_R(crate::FieldReader<bool, bool>);
impl PLLRDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYC` reader - HSE ready Interrupt clear"]
pub struct HSERDYC_R(crate::FieldReader<bool, bool>);
impl HSERDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI16RDYC` reader - HSI16 ready Interrupt clear"]
pub struct HSI16RDYC_R(crate::FieldReader<bool, bool>);
impl HSI16RDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSI16RDYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI16RDYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYC` reader - LSE ready Interrupt clear"]
pub struct LSERDYC_R(crate::FieldReader<bool, bool>);
impl LSERDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYC` reader - LSI ready Interrupt clear"]
pub struct LSIRDYC_R(crate::FieldReader<bool, bool>);
impl LSIRDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - Clock Security System Interrupt clear"]
    #[inline(always)]
    pub fn csshsec(&self) -> CSSHSEC_R {
        CSSHSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt clear"]
    #[inline(always)]
    pub fn csslsec(&self) -> CSSLSEC_R {
        CSSLSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready Interrupt clear"]
    #[inline(always)]
    pub fn msirdyc(&self) -> MSIRDYC_R {
        MSIRDYC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready Interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&self) -> PLLRDYC_R {
        PLLRDYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready Interrupt clear"]
    #[inline(always)]
    pub fn hsi16rdyc(&self) -> HSI16RDYC_R {
        HSI16RDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LSI ready Interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clock interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](index.html) module"]
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cicr::R](R) reader structure"]
impl crate::Readable for CICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
