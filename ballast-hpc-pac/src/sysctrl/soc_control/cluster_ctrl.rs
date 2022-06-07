#[doc = "Register `CLUSTER_CTRL` reader"]
pub struct R(crate::R<CLUSTER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLUSTER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLUSTER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLUSTER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLUSTER_CTRL` writer"]
pub struct W(crate::W<CLUSTER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUSTER_CTRL_SPEC>;
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
impl From<crate::W<CLUSTER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUSTER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bypass` reader - "]
pub struct BYPASS_R(crate::FieldReader<bool>);
impl BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bypass` writer - "]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Field `POW` reader - "]
pub struct POW_R(crate::FieldReader<bool>);
impl POW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POW` writer - "]
pub struct POW_W<'a> {
    w: &'a mut W,
}
impl<'a> POW_W<'a> {
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
#[doc = "Field `fetch_enable` reader - "]
pub struct FETCH_ENABLE_R(crate::FieldReader<bool>);
impl FETCH_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FETCH_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FETCH_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fetch_enable` writer - "]
pub struct FETCH_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FETCH_ENABLE_W<'a> {
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
#[doc = "Field `rstn` reader - "]
pub struct RSTN_R(crate::FieldReader<bool>);
impl RSTN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rstn` writer - "]
pub struct RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTN_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pow(&self) -> POW_R {
        POW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetch_enable(&self) -> FETCH_ENABLE_R {
        FETCH_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pow(&mut self) -> POW_W {
        POW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetch_enable(&mut self) -> FETCH_ENABLE_W {
        FETCH_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RSTN_W {
        RSTN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cluster_ctrl](index.html) module"]
pub struct CLUSTER_CTRL_SPEC;
impl crate::RegisterSpec for CLUSTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cluster_ctrl::R](R) reader structure"]
impl crate::Readable for CLUSTER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cluster_ctrl::W](W) writer structure"]
impl crate::Writable for CLUSTER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLUSTER_CTRL to value 0x09"]
impl crate::Resettable for CLUSTER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09
    }
}
