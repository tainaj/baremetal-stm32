#[doc = "Register `COMP2_CTRL` reader"]
pub struct R(crate::R<COMP2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CTRL` writer"]
pub struct W(crate::W<COMP2_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CTRL_SPEC>;
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
impl From<crate::W<COMP2_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable bit"]
pub struct COMP2EN_R(crate::FieldReader<bool, bool>);
impl COMP2EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable bit"]
pub struct COMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2EN_W<'a> {
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
#[doc = "Field `COMP2SPEED` reader - Comparator 2 power mode selection bit"]
pub struct COMP2SPEED_R(crate::FieldReader<bool, bool>);
impl COMP2SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2SPEED` writer - Comparator 2 power mode selection bit"]
pub struct COMP2SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2SPEED_W<'a> {
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
#[doc = "Field `COMP2INNSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2INNSEL_R(crate::FieldReader<u8, u8>);
impl COMP2INNSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INNSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INNSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INNSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2INPSEL_R(crate::FieldReader<u8, u8>);
impl COMP2INPSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `COMP2LPTIMIN2` reader - Comparator 2 LPTIM input 2 propagation bit"]
pub struct COMP2LPTIMIN2_R(crate::FieldReader<bool, bool>);
impl COMP2LPTIMIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LPTIMIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2LPTIMIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LPTIMIN2` writer - Comparator 2 LPTIM input 2 propagation bit"]
pub struct COMP2LPTIMIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LPTIMIN2_W<'a> {
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
#[doc = "Field `COMP2LPTIMIN1` reader - Comparator 2 LPTIM input 1 propagation bit"]
pub struct COMP2LPTIMIN1_R(crate::FieldReader<bool, bool>);
impl COMP2LPTIMIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LPTIMIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2LPTIMIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LPTIMIN1` writer - Comparator 2 LPTIM input 1 propagation bit"]
pub struct COMP2LPTIMIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LPTIMIN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `COMP2POLARITY` reader - Comparator 2 polarity selection bit"]
pub struct COMP2POLARITY_R(crate::FieldReader<bool, bool>);
impl COMP2POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2POLARITY` writer - Comparator 2 polarity selection bit"]
pub struct COMP2POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2POLARITY_W<'a> {
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
#[doc = "Field `COMP2VALUE` reader - Comparator 2 output status bit"]
pub struct COMP2VALUE_R(crate::FieldReader<bool, bool>);
impl COMP2VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2VALUE` writer - Comparator 2 output status bit"]
pub struct COMP2VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2VALUE_W<'a> {
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
#[doc = "Field `COMP2LOCK` reader - COMP2_CSR register lock bit"]
pub struct COMP2LOCK_R(crate::FieldReader<bool, bool>);
impl COMP2LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LOCK` writer - COMP2_CSR register lock bit"]
pub struct COMP2LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2speed(&self) -> COMP2SPEED_R {
        COMP2SPEED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2innsel(&self) -> COMP2INNSEL_R {
        COMP2INNSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin2(&self) -> COMP2LPTIMIN2_R {
        COMP2LPTIMIN2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin1(&self) -> COMP2LPTIMIN1_R {
        COMP2LPTIMIN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2polarity(&self) -> COMP2POLARITY_R {
        COMP2POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2value(&self) -> COMP2VALUE_R {
        COMP2VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W {
        COMP2EN_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2speed(&mut self) -> COMP2SPEED_W {
        COMP2SPEED_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2innsel(&mut self) -> COMP2INNSEL_W {
        COMP2INNSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W {
        COMP2INPSEL_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin2(&mut self) -> COMP2LPTIMIN2_W {
        COMP2LPTIMIN2_W { w: self }
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin1(&mut self) -> COMP2LPTIMIN1_W {
        COMP2LPTIMIN1_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2polarity(&mut self) -> COMP2POLARITY_W {
        COMP2POLARITY_W { w: self }
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2value(&mut self) -> COMP2VALUE_W {
        COMP2VALUE_W { w: self }
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W {
        COMP2LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_ctrl](index.html) module"]
pub struct COMP2_CTRL_SPEC;
impl crate::RegisterSpec for COMP2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_ctrl::R](R) reader structure"]
impl crate::Readable for COMP2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_ctrl::W](W) writer structure"]
impl crate::Writable for COMP2_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CTRL to value 0"]
impl crate::Resettable for COMP2_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
