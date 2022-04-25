#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OD15` reader - Port output data bit (y = 0..15)"]
pub struct OD15_R(crate::FieldReader<bool, bool>);
impl OD15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD15` writer - Port output data bit (y = 0..15)"]
pub struct OD15_W<'a> {
    w: &'a mut W,
}
impl<'a> OD15_W<'a> {
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
#[doc = "Field `OD14` reader - Port output data bit (y = 0..15)"]
pub struct OD14_R(crate::FieldReader<bool, bool>);
impl OD14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD14` writer - Port output data bit (y = 0..15)"]
pub struct OD14_W<'a> {
    w: &'a mut W,
}
impl<'a> OD14_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `OD13` reader - Port output data bit (y = 0..15)"]
pub struct OD13_R(crate::FieldReader<bool, bool>);
impl OD13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD13` writer - Port output data bit (y = 0..15)"]
pub struct OD13_W<'a> {
    w: &'a mut W,
}
impl<'a> OD13_W<'a> {
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
#[doc = "Field `OD12` reader - Port output data bit (y = 0..15)"]
pub struct OD12_R(crate::FieldReader<bool, bool>);
impl OD12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD12` writer - Port output data bit (y = 0..15)"]
pub struct OD12_W<'a> {
    w: &'a mut W,
}
impl<'a> OD12_W<'a> {
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
#[doc = "Field `OD11` reader - Port output data bit (y = 0..15)"]
pub struct OD11_R(crate::FieldReader<bool, bool>);
impl OD11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD11` writer - Port output data bit (y = 0..15)"]
pub struct OD11_W<'a> {
    w: &'a mut W,
}
impl<'a> OD11_W<'a> {
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
#[doc = "Field `OD10` reader - Port output data bit (y = 0..15)"]
pub struct OD10_R(crate::FieldReader<bool, bool>);
impl OD10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD10` writer - Port output data bit (y = 0..15)"]
pub struct OD10_W<'a> {
    w: &'a mut W,
}
impl<'a> OD10_W<'a> {
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
#[doc = "Field `OD9` reader - Port output data bit (y = 0..15)"]
pub struct OD9_R(crate::FieldReader<bool, bool>);
impl OD9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD9` writer - Port output data bit (y = 0..15)"]
pub struct OD9_W<'a> {
    w: &'a mut W,
}
impl<'a> OD9_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `OD8` reader - Port output data bit (y = 0..15)"]
pub struct OD8_R(crate::FieldReader<bool, bool>);
impl OD8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD8` writer - Port output data bit (y = 0..15)"]
pub struct OD8_W<'a> {
    w: &'a mut W,
}
impl<'a> OD8_W<'a> {
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
#[doc = "Field `OD7` reader - Port output data bit (y = 0..15)"]
pub struct OD7_R(crate::FieldReader<bool, bool>);
impl OD7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD7` writer - Port output data bit (y = 0..15)"]
pub struct OD7_W<'a> {
    w: &'a mut W,
}
impl<'a> OD7_W<'a> {
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
#[doc = "Field `OD6` reader - Port output data bit (y = 0..15)"]
pub struct OD6_R(crate::FieldReader<bool, bool>);
impl OD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD6` writer - Port output data bit (y = 0..15)"]
pub struct OD6_W<'a> {
    w: &'a mut W,
}
impl<'a> OD6_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OD5` reader - Port output data bit (y = 0..15)"]
pub struct OD5_R(crate::FieldReader<bool, bool>);
impl OD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD5` writer - Port output data bit (y = 0..15)"]
pub struct OD5_W<'a> {
    w: &'a mut W,
}
impl<'a> OD5_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `OD4` reader - Port output data bit (y = 0..15)"]
pub struct OD4_R(crate::FieldReader<bool, bool>);
impl OD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD4` writer - Port output data bit (y = 0..15)"]
pub struct OD4_W<'a> {
    w: &'a mut W,
}
impl<'a> OD4_W<'a> {
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
#[doc = "Field `OD3` reader - Port output data bit (y = 0..15)"]
pub struct OD3_R(crate::FieldReader<bool, bool>);
impl OD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD3` writer - Port output data bit (y = 0..15)"]
pub struct OD3_W<'a> {
    w: &'a mut W,
}
impl<'a> OD3_W<'a> {
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
#[doc = "Field `OD2` reader - Port output data bit (y = 0..15)"]
pub struct OD2_R(crate::FieldReader<bool, bool>);
impl OD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD2` writer - Port output data bit (y = 0..15)"]
pub struct OD2_W<'a> {
    w: &'a mut W,
}
impl<'a> OD2_W<'a> {
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
#[doc = "Field `OD1` reader - Port output data bit (y = 0..15)"]
pub struct OD1_R(crate::FieldReader<bool, bool>);
impl OD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD1` writer - Port output data bit (y = 0..15)"]
pub struct OD1_W<'a> {
    w: &'a mut W,
}
impl<'a> OD1_W<'a> {
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
#[doc = "Field `OD0` reader - Port output data bit (y = 0..15)"]
pub struct OD0_R(crate::FieldReader<bool, bool>);
impl OD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD0` writer - Port output data bit (y = 0..15)"]
pub struct OD0_W<'a> {
    w: &'a mut W,
}
impl<'a> OD0_W<'a> {
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
impl R {
    #[doc = "Bit 15 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od15(&mut self) -> OD15_W {
        OD15_W { w: self }
    }
    #[doc = "Bit 14 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od14(&mut self) -> OD14_W {
        OD14_W { w: self }
    }
    #[doc = "Bit 13 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od13(&mut self) -> OD13_W {
        OD13_W { w: self }
    }
    #[doc = "Bit 12 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od12(&mut self) -> OD12_W {
        OD12_W { w: self }
    }
    #[doc = "Bit 11 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od11(&mut self) -> OD11_W {
        OD11_W { w: self }
    }
    #[doc = "Bit 10 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od10(&mut self) -> OD10_W {
        OD10_W { w: self }
    }
    #[doc = "Bit 9 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od9(&mut self) -> OD9_W {
        OD9_W { w: self }
    }
    #[doc = "Bit 8 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&mut self) -> OD8_W {
        OD8_W { w: self }
    }
    #[doc = "Bit 7 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&mut self) -> OD7_W {
        OD7_W { w: self }
    }
    #[doc = "Bit 6 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&mut self) -> OD6_W {
        OD6_W { w: self }
    }
    #[doc = "Bit 5 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&mut self) -> OD5_W {
        OD5_W { w: self }
    }
    #[doc = "Bit 4 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&mut self) -> OD4_W {
        OD4_W { w: self }
    }
    #[doc = "Bit 3 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&mut self) -> OD3_W {
        OD3_W { w: self }
    }
    #[doc = "Bit 2 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&mut self) -> OD2_W {
        OD2_W { w: self }
    }
    #[doc = "Bit 1 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&mut self) -> OD1_W {
        OD1_W { w: self }
    }
    #[doc = "Bit 0 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&mut self) -> OD0_W {
        OD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
