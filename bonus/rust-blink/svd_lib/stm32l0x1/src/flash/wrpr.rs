#[doc = "Register `WRPR` reader"]
pub struct R(crate::R<WRPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPR` writer"]
pub struct W(crate::W<WRPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPR_SPEC>;
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
impl From<crate::W<WRPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP` reader - Write protection"]
pub struct WRP_R(crate::FieldReader<u16, u16>);
impl WRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRP` writer - Write protection"]
pub struct WRP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write protection"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write protection"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WRP_W {
        WRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpr](index.html) module"]
pub struct WRPR_SPEC;
impl crate::RegisterSpec for WRPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpr::R](R) reader structure"]
impl crate::Readable for WRPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpr::W](W) writer structure"]
impl crate::Writable for WRPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRPR to value 0"]
impl crate::Resettable for WRPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
