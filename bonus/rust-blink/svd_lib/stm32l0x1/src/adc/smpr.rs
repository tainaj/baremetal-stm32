#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMPR` reader - Sampling time selection"]
pub struct SMPR_R(crate::FieldReader<u8, u8>);
impl SMPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPR` writer - Sampling time selection"]
pub struct SMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smpr(&self) -> SMPR_R {
        SMPR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smpr(&mut self) -> SMPR_W {
        SMPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
