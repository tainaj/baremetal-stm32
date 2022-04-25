#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM1SEL1` reader - Low Power Timer clock source selection bits"]
pub struct LPTIM1SEL1_R(crate::FieldReader<bool, bool>);
impl LPTIM1SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1SEL1` writer - Low Power Timer clock source selection bits"]
pub struct LPTIM1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `LPTIM1SEL0` reader - LPTIM1SEL0"]
pub struct LPTIM1SEL0_R(crate::FieldReader<bool, bool>);
impl LPTIM1SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1SEL0` writer - LPTIM1SEL0"]
pub struct LPTIM1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `I2C1SEL1` reader - I2C1 clock source selection bits"]
pub struct I2C1SEL1_R(crate::FieldReader<bool, bool>);
impl I2C1SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SEL1` writer - I2C1 clock source selection bits"]
pub struct I2C1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL1_W<'a> {
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
#[doc = "Field `I2C1SEL0` reader - I2C1SEL0"]
pub struct I2C1SEL0_R(crate::FieldReader<bool, bool>);
impl I2C1SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SEL0` writer - I2C1SEL0"]
pub struct I2C1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL0_W<'a> {
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
#[doc = "Field `LPUART1SEL1` reader - LPUART1 clock source selection bits"]
pub struct LPUART1SEL1_R(crate::FieldReader<bool, bool>);
impl LPUART1SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SEL1` writer - LPUART1 clock source selection bits"]
pub struct LPUART1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `LPUART1SEL0` reader - LPUART1SEL0"]
pub struct LPUART1SEL0_R(crate::FieldReader<bool, bool>);
impl LPUART1SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SEL0` writer - LPUART1SEL0"]
pub struct LPUART1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `USART2SEL1` reader - USART2 clock source selection bits"]
pub struct USART2SEL1_R(crate::FieldReader<bool, bool>);
impl USART2SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USART2SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SEL1` writer - USART2 clock source selection bits"]
pub struct USART2SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL1_W<'a> {
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
#[doc = "Field `USART2SEL0` reader - USART2SEL0"]
pub struct USART2SEL0_R(crate::FieldReader<bool, bool>);
impl USART2SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USART2SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SEL0` writer - USART2SEL0"]
pub struct USART2SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL0_W<'a> {
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
#[doc = "Field `USART1SEL1` reader - USART1 clock source selection bits"]
pub struct USART1SEL1_R(crate::FieldReader<bool, bool>);
impl USART1SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USART1SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEL1` writer - USART1 clock source selection bits"]
pub struct USART1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL1_W<'a> {
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
#[doc = "Field `USART1SEL0` reader - USART1SEL0"]
pub struct USART1SEL0_R(crate::FieldReader<bool, bool>);
impl USART1SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USART1SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEL0` writer - USART1SEL0"]
pub struct USART1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL0_W<'a> {
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
#[doc = "Field `I2C3SEL0` reader - I2C3 clock source selection bits"]
pub struct I2C3SEL0_R(crate::FieldReader<bool, bool>);
impl I2C3SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C3SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3SEL0` writer - I2C3 clock source selection bits"]
pub struct I2C3SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `I2C3SEL1` reader - I2C3 clock source selection bits"]
pub struct I2C3SEL1_R(crate::FieldReader<bool, bool>);
impl I2C3SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C3SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3SEL1` writer - I2C3 clock source selection bits"]
pub struct I2C3SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel1(&self) -> LPTIM1SEL1_R {
        LPTIM1SEL1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - LPTIM1SEL0"]
    #[inline(always)]
    pub fn lptim1sel0(&self) -> LPTIM1SEL0_R {
        LPTIM1SEL0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel1(&self) -> I2C1SEL1_R {
        I2C1SEL1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C1SEL0"]
    #[inline(always)]
    pub fn i2c1sel0(&self) -> I2C1SEL0_R {
        I2C1SEL0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel1(&self) -> LPUART1SEL1_R {
        LPUART1SEL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - LPUART1SEL0"]
    #[inline(always)]
    pub fn lpuart1sel0(&self) -> LPUART1SEL0_R {
        LPUART1SEL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel1(&self) -> USART2SEL1_R {
        USART2SEL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - USART2SEL0"]
    #[inline(always)]
    pub fn usart2sel0(&self) -> USART2SEL0_R {
        USART2SEL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel1(&self) -> USART1SEL1_R {
        USART1SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - USART1SEL0"]
    #[inline(always)]
    pub fn usart1sel0(&self) -> USART1SEL0_R {
        USART1SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel0(&self) -> I2C3SEL0_R {
        I2C3SEL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel1(&self) -> I2C3SEL1_R {
        I2C3SEL1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel1(&mut self) -> LPTIM1SEL1_W {
        LPTIM1SEL1_W { w: self }
    }
    #[doc = "Bit 18 - LPTIM1SEL0"]
    #[inline(always)]
    pub fn lptim1sel0(&mut self) -> LPTIM1SEL0_W {
        LPTIM1SEL0_W { w: self }
    }
    #[doc = "Bit 13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel1(&mut self) -> I2C1SEL1_W {
        I2C1SEL1_W { w: self }
    }
    #[doc = "Bit 12 - I2C1SEL0"]
    #[inline(always)]
    pub fn i2c1sel0(&mut self) -> I2C1SEL0_W {
        I2C1SEL0_W { w: self }
    }
    #[doc = "Bit 11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel1(&mut self) -> LPUART1SEL1_W {
        LPUART1SEL1_W { w: self }
    }
    #[doc = "Bit 10 - LPUART1SEL0"]
    #[inline(always)]
    pub fn lpuart1sel0(&mut self) -> LPUART1SEL0_W {
        LPUART1SEL0_W { w: self }
    }
    #[doc = "Bit 3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel1(&mut self) -> USART2SEL1_W {
        USART2SEL1_W { w: self }
    }
    #[doc = "Bit 2 - USART2SEL0"]
    #[inline(always)]
    pub fn usart2sel0(&mut self) -> USART2SEL0_W {
        USART2SEL0_W { w: self }
    }
    #[doc = "Bit 1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel1(&mut self) -> USART1SEL1_W {
        USART1SEL1_W { w: self }
    }
    #[doc = "Bit 0 - USART1SEL0"]
    #[inline(always)]
    pub fn usart1sel0(&mut self) -> USART1SEL0_W {
        USART1SEL0_W { w: self }
    }
    #[doc = "Bit 16 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel0(&mut self) -> I2C3SEL0_W {
        I2C3SEL0_W { w: self }
    }
    #[doc = "Bit 17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel1(&mut self) -> I2C3SEL1_W {
        I2C3SEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
