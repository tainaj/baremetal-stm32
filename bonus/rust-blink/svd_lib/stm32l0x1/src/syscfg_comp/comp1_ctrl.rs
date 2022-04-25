#[doc = "Register `COMP1_CTRL` reader"]
pub struct R(crate::R<COMP1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CTRL` writer"]
pub struct W(crate::W<COMP1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CTRL_SPEC>;
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
impl From<crate::W<COMP1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1EN` reader - Comparator 1 enable bit"]
pub struct COMP1EN_R(crate::FieldReader<bool, bool>);
impl COMP1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable bit"]
pub struct COMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1EN_W<'a> {
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
#[doc = "Field `COMP1INNSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub struct COMP1INNSEL_R(crate::FieldReader<u8, u8>);
impl COMP1INNSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMP1INNSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1INNSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1INNSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub struct COMP1INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1INNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `COMP1WM` reader - Comparator 1 window mode selection bit"]
pub struct COMP1WM_R(crate::FieldReader<bool, bool>);
impl COMP1WM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1WM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1WM` writer - Comparator 1 window mode selection bit"]
pub struct COMP1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1WM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `COMP1LPTIMIN1` reader - Comparator 1 LPTIM input propagation bit"]
pub struct COMP1LPTIMIN1_R(crate::FieldReader<bool, bool>);
impl COMP1LPTIMIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1LPTIMIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1LPTIMIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1LPTIMIN1` writer - Comparator 1 LPTIM input propagation bit"]
pub struct COMP1LPTIMIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LPTIMIN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `COMP1POLARITY` reader - Comparator 1 polarity selection bit"]
pub struct COMP1POLARITY_R(crate::FieldReader<bool, bool>);
impl COMP1POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1POLARITY` writer - Comparator 1 polarity selection bit"]
pub struct COMP1POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `COMP1VALUE` reader - Comparator 1 output status bit"]
pub struct COMP1VALUE_R(crate::FieldReader<bool, bool>);
impl COMP1VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1VALUE` writer - Comparator 1 output status bit"]
pub struct COMP1VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1VALUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `COMP1LOCK` reader - COMP1_CSR register lock bit"]
pub struct COMP1LOCK_R(crate::FieldReader<bool, bool>);
impl COMP1LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1LOCK` writer - COMP1_CSR register lock bit"]
pub struct COMP1LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&self) -> COMP1INNSEL_R {
        COMP1INNSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&self) -> COMP1WM_R {
        COMP1WM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&self) -> COMP1LPTIMIN1_R {
        COMP1LPTIMIN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&self) -> COMP1POLARITY_R {
        COMP1POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&self) -> COMP1VALUE_R {
        COMP1VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W {
        COMP1EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&mut self) -> COMP1INNSEL_W {
        COMP1INNSEL_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&mut self) -> COMP1WM_W {
        COMP1WM_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&mut self) -> COMP1LPTIMIN1_W {
        COMP1LPTIMIN1_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&mut self) -> COMP1POLARITY_W {
        COMP1POLARITY_W { w: self }
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&mut self) -> COMP1VALUE_W {
        COMP1VALUE_W { w: self }
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W {
        COMP1LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_ctrl](index.html) module"]
pub struct COMP1_CTRL_SPEC;
impl crate::RegisterSpec for COMP1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_ctrl::R](R) reader structure"]
impl crate::Readable for COMP1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_ctrl::W](W) writer structure"]
impl crate::Writable for COMP1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP1_CTRL to value 0"]
impl crate::Resettable for COMP1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
