#[doc = "Register `RTSR` reader"]
pub struct R(crate::R<RTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTSR` writer"]
pub struct W(crate::W<RTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR_SPEC>;
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
impl From<crate::W<RTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT0` reader - Rising trigger event configuration of line 0"]
pub struct RT0_R(crate::FieldReader<bool, bool>);
impl RT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0` writer - Rising trigger event configuration of line 0"]
pub struct RT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0_W<'a> {
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
#[doc = "Field `RT1` reader - Rising trigger event configuration of line 1"]
pub struct RT1_R(crate::FieldReader<bool, bool>);
impl RT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT1` writer - Rising trigger event configuration of line 1"]
pub struct RT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1_W<'a> {
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
#[doc = "Field `RT2` reader - Rising trigger event configuration of line 2"]
pub struct RT2_R(crate::FieldReader<bool, bool>);
impl RT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT2` writer - Rising trigger event configuration of line 2"]
pub struct RT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RT2_W<'a> {
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
#[doc = "Field `RT3` reader - Rising trigger event configuration of line 3"]
pub struct RT3_R(crate::FieldReader<bool, bool>);
impl RT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT3` writer - Rising trigger event configuration of line 3"]
pub struct RT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RT3_W<'a> {
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
#[doc = "Field `RT4` reader - Rising trigger event configuration of line 4"]
pub struct RT4_R(crate::FieldReader<bool, bool>);
impl RT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT4` writer - Rising trigger event configuration of line 4"]
pub struct RT4_W<'a> {
    w: &'a mut W,
}
impl<'a> RT4_W<'a> {
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
#[doc = "Field `RT5` reader - Rising trigger event configuration of line 5"]
pub struct RT5_R(crate::FieldReader<bool, bool>);
impl RT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT5` writer - Rising trigger event configuration of line 5"]
pub struct RT5_W<'a> {
    w: &'a mut W,
}
impl<'a> RT5_W<'a> {
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
#[doc = "Field `RT6` reader - Rising trigger event configuration of line 6"]
pub struct RT6_R(crate::FieldReader<bool, bool>);
impl RT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT6` writer - Rising trigger event configuration of line 6"]
pub struct RT6_W<'a> {
    w: &'a mut W,
}
impl<'a> RT6_W<'a> {
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
#[doc = "Field `RT7` reader - Rising trigger event configuration of line 7"]
pub struct RT7_R(crate::FieldReader<bool, bool>);
impl RT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT7` writer - Rising trigger event configuration of line 7"]
pub struct RT7_W<'a> {
    w: &'a mut W,
}
impl<'a> RT7_W<'a> {
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
#[doc = "Field `RT8` reader - Rising trigger event configuration of line 8"]
pub struct RT8_R(crate::FieldReader<bool, bool>);
impl RT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT8` writer - Rising trigger event configuration of line 8"]
pub struct RT8_W<'a> {
    w: &'a mut W,
}
impl<'a> RT8_W<'a> {
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
#[doc = "Field `RT9` reader - Rising trigger event configuration of line 9"]
pub struct RT9_R(crate::FieldReader<bool, bool>);
impl RT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT9` writer - Rising trigger event configuration of line 9"]
pub struct RT9_W<'a> {
    w: &'a mut W,
}
impl<'a> RT9_W<'a> {
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
#[doc = "Field `RT10` reader - Rising trigger event configuration of line 10"]
pub struct RT10_R(crate::FieldReader<bool, bool>);
impl RT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT10` writer - Rising trigger event configuration of line 10"]
pub struct RT10_W<'a> {
    w: &'a mut W,
}
impl<'a> RT10_W<'a> {
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
#[doc = "Field `RT11` reader - Rising trigger event configuration of line 11"]
pub struct RT11_R(crate::FieldReader<bool, bool>);
impl RT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT11` writer - Rising trigger event configuration of line 11"]
pub struct RT11_W<'a> {
    w: &'a mut W,
}
impl<'a> RT11_W<'a> {
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
#[doc = "Field `RT12` reader - Rising trigger event configuration of line 12"]
pub struct RT12_R(crate::FieldReader<bool, bool>);
impl RT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT12` writer - Rising trigger event configuration of line 12"]
pub struct RT12_W<'a> {
    w: &'a mut W,
}
impl<'a> RT12_W<'a> {
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
#[doc = "Field `RT13` reader - Rising trigger event configuration of line 13"]
pub struct RT13_R(crate::FieldReader<bool, bool>);
impl RT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT13` writer - Rising trigger event configuration of line 13"]
pub struct RT13_W<'a> {
    w: &'a mut W,
}
impl<'a> RT13_W<'a> {
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
#[doc = "Field `RT14` reader - Rising trigger event configuration of line 14"]
pub struct RT14_R(crate::FieldReader<bool, bool>);
impl RT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT14` writer - Rising trigger event configuration of line 14"]
pub struct RT14_W<'a> {
    w: &'a mut W,
}
impl<'a> RT14_W<'a> {
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
#[doc = "Field `RT15` reader - Rising trigger event configuration of line 15"]
pub struct RT15_R(crate::FieldReader<bool, bool>);
impl RT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT15` writer - Rising trigger event configuration of line 15"]
pub struct RT15_W<'a> {
    w: &'a mut W,
}
impl<'a> RT15_W<'a> {
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
#[doc = "Field `RT16` reader - Rising trigger event configuration of line 16"]
pub struct RT16_R(crate::FieldReader<bool, bool>);
impl RT16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT16` writer - Rising trigger event configuration of line 16"]
pub struct RT16_W<'a> {
    w: &'a mut W,
}
impl<'a> RT16_W<'a> {
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
#[doc = "Field `RT17` reader - Rising trigger event configuration of line 17"]
pub struct RT17_R(crate::FieldReader<bool, bool>);
impl RT17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT17` writer - Rising trigger event configuration of line 17"]
pub struct RT17_W<'a> {
    w: &'a mut W,
}
impl<'a> RT17_W<'a> {
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
#[doc = "Field `RT19` reader - Rising trigger event configuration of line 19"]
pub struct RT19_R(crate::FieldReader<bool, bool>);
impl RT19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT19` writer - Rising trigger event configuration of line 19"]
pub struct RT19_W<'a> {
    w: &'a mut W,
}
impl<'a> RT19_W<'a> {
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
#[doc = "Field `RT20` reader - Rising trigger event configuration of line 20"]
pub struct RT20_R(crate::FieldReader<bool, bool>);
impl RT20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT20` writer - Rising trigger event configuration of line 20"]
pub struct RT20_W<'a> {
    w: &'a mut W,
}
impl<'a> RT20_W<'a> {
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
#[doc = "Field `RT21` reader - Rising trigger event configuration of line 21"]
pub struct RT21_R(crate::FieldReader<bool, bool>);
impl RT21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT21` writer - Rising trigger event configuration of line 21"]
pub struct RT21_W<'a> {
    w: &'a mut W,
}
impl<'a> RT21_W<'a> {
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
#[doc = "Field `RT22` reader - Rising trigger event configuration of line 22"]
pub struct RT22_R(crate::FieldReader<bool, bool>);
impl RT22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT22` writer - Rising trigger event configuration of line 22"]
pub struct RT22_W<'a> {
    w: &'a mut W,
}
impl<'a> RT22_W<'a> {
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
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn rt17(&self) -> RT17_R {
        RT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub fn rt19(&self) -> RT19_R {
        RT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration of line 20"]
    #[inline(always)]
    pub fn rt20(&self) -> RT20_R {
        RT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub fn rt22(&self) -> RT22_R {
        RT22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W {
        RT0_W { w: self }
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W {
        RT1_W { w: self }
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W {
        RT2_W { w: self }
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W {
        RT3_W { w: self }
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W {
        RT4_W { w: self }
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W {
        RT5_W { w: self }
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W {
        RT6_W { w: self }
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W {
        RT7_W { w: self }
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W {
        RT8_W { w: self }
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W {
        RT9_W { w: self }
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W {
        RT10_W { w: self }
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W {
        RT11_W { w: self }
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W {
        RT12_W { w: self }
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W {
        RT13_W { w: self }
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W {
        RT14_W { w: self }
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W {
        RT15_W { w: self }
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn rt16(&mut self) -> RT16_W {
        RT16_W { w: self }
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn rt17(&mut self) -> RT17_W {
        RT17_W { w: self }
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub fn rt19(&mut self) -> RT19_W {
        RT19_W { w: self }
    }
    #[doc = "Bit 20 - Rising trigger event configuration of line 20"]
    #[inline(always)]
    pub fn rt20(&mut self) -> RT20_W {
        RT20_W { w: self }
    }
    #[doc = "Bit 21 - Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub fn rt21(&mut self) -> RT21_W {
        RT21_W { w: self }
    }
    #[doc = "Bit 22 - Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub fn rt22(&mut self) -> RT22_W {
        RT22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rising Trigger selection register (EXTI_RTSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr](index.html) module"]
pub struct RTSR_SPEC;
impl crate::RegisterSpec for RTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtsr::R](R) reader structure"]
impl crate::Readable for RTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtsr::W](W) writer structure"]
impl crate::Writable for RTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
