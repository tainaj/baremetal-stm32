#[doc = "Register `FIREWALL_CR` reader"]
pub struct R(crate::R<FIREWALL_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIREWALL_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIREWALL_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIREWALL_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIREWALL_CR` writer"]
pub struct W(crate::W<FIREWALL_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIREWALL_CR_SPEC>;
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
impl From<crate::W<FIREWALL_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIREWALL_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDE` reader - Volatile data execution"]
pub struct VDE_R(crate::FieldReader<bool, bool>);
impl VDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDE` writer - Volatile data execution"]
pub struct VDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDE_W<'a> {
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
#[doc = "Field `VDS` reader - Volatile data shared"]
pub struct VDS_R(crate::FieldReader<bool, bool>);
impl VDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDS` writer - Volatile data shared"]
pub struct VDS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDS_W<'a> {
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
#[doc = "Field `FPA` reader - Firewall pre alarm"]
pub struct FPA_R(crate::FieldReader<bool, bool>);
impl FPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPA` writer - Firewall pre alarm"]
pub struct FPA_W<'a> {
    w: &'a mut W,
}
impl<'a> FPA_W<'a> {
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
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&self) -> VDS_R {
        VDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&self) -> FPA_R {
        FPA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&mut self) -> VDE_W {
        VDE_W { w: self }
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&mut self) -> VDS_W {
        VDS_W { w: self }
    }
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&mut self) -> FPA_W {
        FPA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_cr](index.html) module"]
pub struct FIREWALL_CR_SPEC;
impl crate::RegisterSpec for FIREWALL_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [firewall_cr::R](R) reader structure"]
impl crate::Readable for FIREWALL_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [firewall_cr::W](W) writer structure"]
impl crate::Writable for FIREWALL_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIREWALL_CR to value 0"]
impl crate::Resettable for FIREWALL_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
