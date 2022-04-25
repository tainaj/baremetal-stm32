#[doc = "Register `CHSELR` reader"]
pub struct R(crate::R<CHSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR` writer"]
pub struct W(crate::W<CHSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR_SPEC>;
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
impl From<crate::W<CHSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL18` reader - Channel-x selection"]
pub struct CHSEL18_R(crate::FieldReader<bool, bool>);
impl CHSEL18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL18` writer - Channel-x selection"]
pub struct CHSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL18_W<'a> {
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
#[doc = "Field `CHSEL17` reader - Channel-x selection"]
pub struct CHSEL17_R(crate::FieldReader<bool, bool>);
impl CHSEL17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL17` writer - Channel-x selection"]
pub struct CHSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL17_W<'a> {
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
#[doc = "Field `CHSEL16` reader - Channel-x selection"]
pub struct CHSEL16_R(crate::FieldReader<bool, bool>);
impl CHSEL16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL16` writer - Channel-x selection"]
pub struct CHSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL16_W<'a> {
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
#[doc = "Field `CHSEL15` reader - Channel-x selection"]
pub struct CHSEL15_R(crate::FieldReader<bool, bool>);
impl CHSEL15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL15` writer - Channel-x selection"]
pub struct CHSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL15_W<'a> {
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
#[doc = "Field `CHSEL14` reader - Channel-x selection"]
pub struct CHSEL14_R(crate::FieldReader<bool, bool>);
impl CHSEL14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL14` writer - Channel-x selection"]
pub struct CHSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL14_W<'a> {
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
#[doc = "Field `CHSEL13` reader - Channel-x selection"]
pub struct CHSEL13_R(crate::FieldReader<bool, bool>);
impl CHSEL13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL13` writer - Channel-x selection"]
pub struct CHSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL13_W<'a> {
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
#[doc = "Field `CHSEL12` reader - Channel-x selection"]
pub struct CHSEL12_R(crate::FieldReader<bool, bool>);
impl CHSEL12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL12` writer - Channel-x selection"]
pub struct CHSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL12_W<'a> {
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
#[doc = "Field `CHSEL11` reader - Channel-x selection"]
pub struct CHSEL11_R(crate::FieldReader<bool, bool>);
impl CHSEL11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL11` writer - Channel-x selection"]
pub struct CHSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL11_W<'a> {
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
#[doc = "Field `CHSEL10` reader - Channel-x selection"]
pub struct CHSEL10_R(crate::FieldReader<bool, bool>);
impl CHSEL10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL10` writer - Channel-x selection"]
pub struct CHSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL10_W<'a> {
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
#[doc = "Field `CHSEL9` reader - Channel-x selection"]
pub struct CHSEL9_R(crate::FieldReader<bool, bool>);
impl CHSEL9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL9` writer - Channel-x selection"]
pub struct CHSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL9_W<'a> {
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
#[doc = "Field `CHSEL8` reader - Channel-x selection"]
pub struct CHSEL8_R(crate::FieldReader<bool, bool>);
impl CHSEL8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL8` writer - Channel-x selection"]
pub struct CHSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL8_W<'a> {
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
#[doc = "Field `CHSEL7` reader - Channel-x selection"]
pub struct CHSEL7_R(crate::FieldReader<bool, bool>);
impl CHSEL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL7` writer - Channel-x selection"]
pub struct CHSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL7_W<'a> {
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
#[doc = "Field `CHSEL6` reader - Channel-x selection"]
pub struct CHSEL6_R(crate::FieldReader<bool, bool>);
impl CHSEL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL6` writer - Channel-x selection"]
pub struct CHSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL6_W<'a> {
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
#[doc = "Field `CHSEL5` reader - Channel-x selection"]
pub struct CHSEL5_R(crate::FieldReader<bool, bool>);
impl CHSEL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL5` writer - Channel-x selection"]
pub struct CHSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL5_W<'a> {
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
#[doc = "Field `CHSEL4` reader - Channel-x selection"]
pub struct CHSEL4_R(crate::FieldReader<bool, bool>);
impl CHSEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL4` writer - Channel-x selection"]
pub struct CHSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL4_W<'a> {
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
#[doc = "Field `CHSEL3` reader - Channel-x selection"]
pub struct CHSEL3_R(crate::FieldReader<bool, bool>);
impl CHSEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL3` writer - Channel-x selection"]
pub struct CHSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL3_W<'a> {
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
#[doc = "Field `CHSEL2` reader - Channel-x selection"]
pub struct CHSEL2_R(crate::FieldReader<bool, bool>);
impl CHSEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL2` writer - Channel-x selection"]
pub struct CHSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL2_W<'a> {
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
#[doc = "Field `CHSEL1` reader - Channel-x selection"]
pub struct CHSEL1_R(crate::FieldReader<bool, bool>);
impl CHSEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL1` writer - Channel-x selection"]
pub struct CHSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL1_W<'a> {
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
#[doc = "Field `CHSEL0` reader - Channel-x selection"]
pub struct CHSEL0_R(crate::FieldReader<bool, bool>);
impl CHSEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL0` writer - Channel-x selection"]
pub struct CHSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL0_W<'a> {
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
    #[doc = "Bit 18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL18_W {
        CHSEL18_W { w: self }
    }
    #[doc = "Bit 17 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL17_W {
        CHSEL17_W { w: self }
    }
    #[doc = "Bit 16 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL16_W {
        CHSEL16_W { w: self }
    }
    #[doc = "Bit 15 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL15_W {
        CHSEL15_W { w: self }
    }
    #[doc = "Bit 14 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL14_W {
        CHSEL14_W { w: self }
    }
    #[doc = "Bit 13 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL13_W {
        CHSEL13_W { w: self }
    }
    #[doc = "Bit 12 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL12_W {
        CHSEL12_W { w: self }
    }
    #[doc = "Bit 11 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL11_W {
        CHSEL11_W { w: self }
    }
    #[doc = "Bit 10 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL10_W {
        CHSEL10_W { w: self }
    }
    #[doc = "Bit 9 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL9_W {
        CHSEL9_W { w: self }
    }
    #[doc = "Bit 8 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL8_W {
        CHSEL8_W { w: self }
    }
    #[doc = "Bit 7 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W {
        CHSEL7_W { w: self }
    }
    #[doc = "Bit 6 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W {
        CHSEL6_W { w: self }
    }
    #[doc = "Bit 5 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W {
        CHSEL5_W { w: self }
    }
    #[doc = "Bit 4 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W {
        CHSEL4_W { w: self }
    }
    #[doc = "Bit 3 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W {
        CHSEL3_W { w: self }
    }
    #[doc = "Bit 2 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W {
        CHSEL2_W { w: self }
    }
    #[doc = "Bit 1 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W {
        CHSEL1_W { w: self }
    }
    #[doc = "Bit 0 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W {
        CHSEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr](index.html) module"]
pub struct CHSELR_SPEC;
impl crate::RegisterSpec for CHSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr::R](R) reader structure"]
impl crate::Readable for CHSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr::W](W) writer structure"]
impl crate::Writable for CHSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSELR to value 0"]
impl crate::Resettable for CHSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
