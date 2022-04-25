#[doc = "Register `IOPSMEN` reader"]
pub struct R(crate::R<IOPSMEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPSMEN` writer"]
pub struct W(crate::W<IOPSMEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMEN_SPEC>;
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
impl From<crate::W<IOPSMEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPHSMEN` reader - Port H clock enable during Sleep mode bit"]
pub struct IOPHSMEN_R(crate::FieldReader<bool, bool>);
impl IOPHSMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOPHSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPHSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPHSMEN` writer - Port H clock enable during Sleep mode bit"]
pub struct IOPHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPHSMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `IOPDSMEN` reader - Port D clock enable during Sleep mode bit"]
pub struct IOPDSMEN_R(crate::FieldReader<bool, bool>);
impl IOPDSMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOPDSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPDSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPDSMEN` writer - Port D clock enable during Sleep mode bit"]
pub struct IOPDSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDSMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `IOPCSMEN` reader - Port C clock enable during Sleep mode bit"]
pub struct IOPCSMEN_R(crate::FieldReader<bool, bool>);
impl IOPCSMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOPCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPCSMEN` writer - Port C clock enable during Sleep mode bit"]
pub struct IOPCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCSMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `IOPBSMEN` reader - Port B clock enable during Sleep mode bit"]
pub struct IOPBSMEN_R(crate::FieldReader<bool, bool>);
impl IOPBSMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOPBSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPBSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPBSMEN` writer - Port B clock enable during Sleep mode bit"]
pub struct IOPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBSMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `IOPASMEN` reader - Port A clock enable during Sleep mode bit"]
pub struct IOPASMEN_R(crate::FieldReader<bool, bool>);
impl IOPASMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOPASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPASMEN` writer - Port A clock enable during Sleep mode bit"]
pub struct IOPASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPASMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `IOPESMEN` reader - Port E clock enable during Sleep mode bit"]
pub struct IOPESMEN_R(crate::FieldReader<bool, bool>);
impl IOPESMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOPESMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPESMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPESMEN` writer - Port E clock enable during Sleep mode bit"]
pub struct IOPESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPESMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Port H clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 3 - Port D clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port C clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port B clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port A clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Port H clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W {
        IOPHSMEN_W { w: self }
    }
    #[doc = "Bit 3 - Port D clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W {
        IOPDSMEN_W { w: self }
    }
    #[doc = "Bit 2 - Port C clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W {
        IOPCSMEN_W { w: self }
    }
    #[doc = "Bit 1 - Port B clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W {
        IOPBSMEN_W { w: self }
    }
    #[doc = "Bit 0 - Port A clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopasmen(&mut self) -> IOPASMEN_W {
        IOPASMEN_W { w: self }
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&mut self) -> IOPESMEN_W {
        IOPESMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmen](index.html) module"]
pub struct IOPSMEN_SPEC;
impl crate::RegisterSpec for IOPSMEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopsmen::R](R) reader structure"]
impl crate::Readable for IOPSMEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopsmen::W](W) writer structure"]
impl crate::Writable for IOPSMEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOPSMEN to value 0x8f"]
impl crate::Resettable for IOPSMEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8f
    }
}
