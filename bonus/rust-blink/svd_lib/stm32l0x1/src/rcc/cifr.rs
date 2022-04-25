#[doc = "Register `CIFR` reader"]
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSSHSEF` reader - Clock Security System Interrupt flag"]
pub struct CSSHSEF_R(crate::FieldReader<bool, bool>);
impl CSSHSEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSHSEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSHSEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSLSEF` reader - LSE Clock Security System Interrupt flag"]
pub struct CSSLSEF_R(crate::FieldReader<bool, bool>);
impl CSSLSEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSLSEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub struct MSIRDYF_R(crate::FieldReader<bool, bool>);
impl MSIRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub struct PLLRDYF_R(crate::FieldReader<bool, bool>);
impl PLLRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub struct HSERDYF_R(crate::FieldReader<bool, bool>);
impl HSERDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI16RDYF` reader - HSI16 ready interrupt flag"]
pub struct HSI16RDYF_R(crate::FieldReader<bool, bool>);
impl HSI16RDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSI16RDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI16RDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub struct LSERDYF_R(crate::FieldReader<bool, bool>);
impl LSERDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub struct LSIRDYF_R(crate::FieldReader<bool, bool>);
impl LSIRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cifr::R](R) reader structure"]
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
