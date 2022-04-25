#[doc = "Register `PDKEYR` writer"]
pub struct W(crate::W<PDKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDKEYR_SPEC>;
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
impl From<crate::W<PDKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDKEYR` writer - RUN_PD in FLASH_ACR key"]
pub struct PDKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - RUN_PD in FLASH_ACR key"]
    #[inline(always)]
    pub fn pdkeyr(&mut self) -> PDKEYR_W {
        PDKEYR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power down key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdkeyr](index.html) module"]
pub struct PDKEYR_SPEC;
impl crate::RegisterSpec for PDKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdkeyr::W](W) writer structure"]
impl crate::Writable for PDKEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDKEYR to value 0"]
impl crate::Resettable for PDKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
