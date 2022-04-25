#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSPEED15` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED15_R(crate::FieldReader<u8, u8>);
impl OSPEED15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED15` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
#[doc = "Field `OSPEED14` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED14_R(crate::FieldReader<u8, u8>);
impl OSPEED14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED14` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `OSPEED13` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED13_R(crate::FieldReader<u8, u8>);
impl OSPEED13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED13` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `OSPEED12` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED12_R(crate::FieldReader<u8, u8>);
impl OSPEED12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED12` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED12_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `OSPEED11` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED11_R(crate::FieldReader<u8, u8>);
impl OSPEED11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED11` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED11_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `OSPEED10` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED10_R(crate::FieldReader<u8, u8>);
impl OSPEED10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED10` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED10_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `OSPEED9` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED9_R(crate::FieldReader<u8, u8>);
impl OSPEED9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED9` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED9_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `OSPEED8` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED8_R(crate::FieldReader<u8, u8>);
impl OSPEED8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED8` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED8_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `OSPEED7` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED7_R(crate::FieldReader<u8, u8>);
impl OSPEED7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED7` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `OSPEED6` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED6_R(crate::FieldReader<u8, u8>);
impl OSPEED6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED6` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `OSPEED5` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED5_R(crate::FieldReader<u8, u8>);
impl OSPEED5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED5` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `OSPEED4` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED4_R(crate::FieldReader<u8, u8>);
impl OSPEED4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED4` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `OSPEED3` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED3_R(crate::FieldReader<u8, u8>);
impl OSPEED3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED3` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `OSPEED2` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED2_R(crate::FieldReader<u8, u8>);
impl OSPEED2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED2` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `OSPEED1` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED1_R(crate::FieldReader<u8, u8>);
impl OSPEED1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED1` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `OSPEED0` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEED0_R(crate::FieldReader<u8, u8>);
impl OSPEED0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEED0` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEED0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W {
        OSPEED15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W {
        OSPEED14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W {
        OSPEED13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED12_W {
        OSPEED12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED11_W {
        OSPEED11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED10_W {
        OSPEED10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED9_W {
        OSPEED9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W {
        OSPEED8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W {
        OSPEED7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W {
        OSPEED6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W {
        OSPEED5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED4_W {
        OSPEED4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W {
        OSPEED3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W {
        OSPEED2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W {
        OSPEED1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W {
        OSPEED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
