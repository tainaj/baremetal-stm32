#[doc = "Register `SWIER` reader"]
pub struct R(crate::R<SWIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER` writer"]
pub struct W(crate::W<SWIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER_SPEC>;
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
impl From<crate::W<SWIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` reader - Software Interrupt on line 0"]
pub struct SWI0_R(crate::FieldReader<bool, bool>);
impl SWI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI0` writer - Software Interrupt on line 0"]
pub struct SWI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI0_W<'a> {
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
#[doc = "Field `SWI1` reader - Software Interrupt on line 1"]
pub struct SWI1_R(crate::FieldReader<bool, bool>);
impl SWI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI1` writer - Software Interrupt on line 1"]
pub struct SWI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI1_W<'a> {
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
#[doc = "Field `SWI2` reader - Software Interrupt on line 2"]
pub struct SWI2_R(crate::FieldReader<bool, bool>);
impl SWI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI2` writer - Software Interrupt on line 2"]
pub struct SWI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI2_W<'a> {
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
#[doc = "Field `SWI3` reader - Software Interrupt on line 3"]
pub struct SWI3_R(crate::FieldReader<bool, bool>);
impl SWI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI3` writer - Software Interrupt on line 3"]
pub struct SWI3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI3_W<'a> {
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
#[doc = "Field `SWI4` reader - Software Interrupt on line 4"]
pub struct SWI4_R(crate::FieldReader<bool, bool>);
impl SWI4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI4` writer - Software Interrupt on line 4"]
pub struct SWI4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI4_W<'a> {
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
#[doc = "Field `SWI5` reader - Software Interrupt on line 5"]
pub struct SWI5_R(crate::FieldReader<bool, bool>);
impl SWI5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI5` writer - Software Interrupt on line 5"]
pub struct SWI5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI5_W<'a> {
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
#[doc = "Field `SWI6` reader - Software Interrupt on line 6"]
pub struct SWI6_R(crate::FieldReader<bool, bool>);
impl SWI6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI6` writer - Software Interrupt on line 6"]
pub struct SWI6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI6_W<'a> {
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
#[doc = "Field `SWI7` reader - Software Interrupt on line 7"]
pub struct SWI7_R(crate::FieldReader<bool, bool>);
impl SWI7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI7` writer - Software Interrupt on line 7"]
pub struct SWI7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI7_W<'a> {
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
#[doc = "Field `SWI8` reader - Software Interrupt on line 8"]
pub struct SWI8_R(crate::FieldReader<bool, bool>);
impl SWI8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI8` writer - Software Interrupt on line 8"]
pub struct SWI8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI8_W<'a> {
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
#[doc = "Field `SWI9` reader - Software Interrupt on line 9"]
pub struct SWI9_R(crate::FieldReader<bool, bool>);
impl SWI9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI9` writer - Software Interrupt on line 9"]
pub struct SWI9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI9_W<'a> {
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
#[doc = "Field `SWI10` reader - Software Interrupt on line 10"]
pub struct SWI10_R(crate::FieldReader<bool, bool>);
impl SWI10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI10` writer - Software Interrupt on line 10"]
pub struct SWI10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI10_W<'a> {
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
#[doc = "Field `SWI11` reader - Software Interrupt on line 11"]
pub struct SWI11_R(crate::FieldReader<bool, bool>);
impl SWI11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI11` writer - Software Interrupt on line 11"]
pub struct SWI11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI11_W<'a> {
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
#[doc = "Field `SWI12` reader - Software Interrupt on line 12"]
pub struct SWI12_R(crate::FieldReader<bool, bool>);
impl SWI12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI12` writer - Software Interrupt on line 12"]
pub struct SWI12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI12_W<'a> {
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
#[doc = "Field `SWI13` reader - Software Interrupt on line 13"]
pub struct SWI13_R(crate::FieldReader<bool, bool>);
impl SWI13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI13` writer - Software Interrupt on line 13"]
pub struct SWI13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI13_W<'a> {
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
#[doc = "Field `SWI14` reader - Software Interrupt on line 14"]
pub struct SWI14_R(crate::FieldReader<bool, bool>);
impl SWI14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI14` writer - Software Interrupt on line 14"]
pub struct SWI14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI14_W<'a> {
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
#[doc = "Field `SWI15` reader - Software Interrupt on line 15"]
pub struct SWI15_R(crate::FieldReader<bool, bool>);
impl SWI15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI15` writer - Software Interrupt on line 15"]
pub struct SWI15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI15_W<'a> {
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
#[doc = "Field `SWI16` reader - Software Interrupt on line 16"]
pub struct SWI16_R(crate::FieldReader<bool, bool>);
impl SWI16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI16` writer - Software Interrupt on line 16"]
pub struct SWI16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI16_W<'a> {
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
#[doc = "Field `SWI17` reader - Software Interrupt on line 17"]
pub struct SWI17_R(crate::FieldReader<bool, bool>);
impl SWI17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI17` writer - Software Interrupt on line 17"]
pub struct SWI17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI17_W<'a> {
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
#[doc = "Field `SWI19` reader - Software Interrupt on line 19"]
pub struct SWI19_R(crate::FieldReader<bool, bool>);
impl SWI19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI19` writer - Software Interrupt on line 19"]
pub struct SWI19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI19_W<'a> {
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
#[doc = "Field `SWI20` reader - Software Interrupt on line 20"]
pub struct SWI20_R(crate::FieldReader<bool, bool>);
impl SWI20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI20` writer - Software Interrupt on line 20"]
pub struct SWI20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI20_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `SWI21` reader - Software Interrupt on line 21"]
pub struct SWI21_R(crate::FieldReader<bool, bool>);
impl SWI21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI21` writer - Software Interrupt on line 21"]
pub struct SWI21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI21_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `SWI22` reader - Software Interrupt on line 22"]
pub struct SWI22_R(crate::FieldReader<bool, bool>);
impl SWI22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI22` writer - Software Interrupt on line 22"]
pub struct SWI22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI22_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swi17(&self) -> SWI17_R {
        SWI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swi19(&self) -> SWI19_R {
        SWI19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swi20(&self) -> SWI20_R {
        SWI20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swi21(&self) -> SWI21_R {
        SWI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swi22(&self) -> SWI22_R {
        SWI22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W {
        SWI0_W { w: self }
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W {
        SWI1_W { w: self }
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W {
        SWI2_W { w: self }
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W {
        SWI3_W { w: self }
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W {
        SWI4_W { w: self }
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W {
        SWI5_W { w: self }
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W {
        SWI6_W { w: self }
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W {
        SWI7_W { w: self }
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W {
        SWI8_W { w: self }
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W {
        SWI9_W { w: self }
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W {
        SWI10_W { w: self }
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W {
        SWI11_W { w: self }
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W {
        SWI12_W { w: self }
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W {
        SWI13_W { w: self }
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W {
        SWI14_W { w: self }
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W {
        SWI15_W { w: self }
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W {
        SWI16_W { w: self }
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swi17(&mut self) -> SWI17_W {
        SWI17_W { w: self }
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swi19(&mut self) -> SWI19_W {
        SWI19_W { w: self }
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swi20(&mut self) -> SWI20_W {
        SWI20_W { w: self }
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swi21(&mut self) -> SWI21_W {
        SWI21_W { w: self }
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swi22(&mut self) -> SWI22_W {
        SWI22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register (EXTI_SWIER)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier](index.html) module"]
pub struct SWIER_SPEC;
impl crate::RegisterSpec for SWIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier::R](R) reader structure"]
impl crate::Readable for SWIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier::W](W) writer structure"]
impl crate::Writable for SWIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
