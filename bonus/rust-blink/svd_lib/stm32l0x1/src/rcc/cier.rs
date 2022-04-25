#[doc = "Register `CIER` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSSLSE` reader - LSE CSS interrupt flag"]
pub struct CSSLSE_R(crate::FieldReader<bool, bool>);
impl CSSLSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSLSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt flag"]
pub struct MSIRDYIE_R(crate::FieldReader<bool, bool>);
impl MSIRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt flag"]
pub struct PLLRDYIE_R(crate::FieldReader<bool, bool>);
impl PLLRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt flag"]
pub struct HSERDYIE_R(crate::FieldReader<bool, bool>);
impl HSERDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI16RDYIE` reader - HSI16 ready interrupt flag"]
pub struct HSI16RDYIE_R(crate::FieldReader<bool, bool>);
impl HSI16RDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSI16RDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI16RDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt flag"]
pub struct LSERDYIE_R(crate::FieldReader<bool, bool>);
impl LSERDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt flag"]
pub struct LSIRDYIE_R(crate::FieldReader<bool, bool>);
impl LSIRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - LSE CSS interrupt flag"]
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
