#[doc = "Register `S_ARBITER` reader"]
pub struct R(crate::R<S_ARBITER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_ARBITER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_ARBITER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_ARBITER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_ARBITER` writer"]
pub struct W(crate::W<S_ARBITER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_ARBITER_SPEC>;
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
impl From<crate::W<S_ARBITER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_ARBITER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARB_WEIGHT` reader - "]
pub struct ARB_WEIGHT_R(crate::FieldReader<u8>);
impl ARB_WEIGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ARB_WEIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_WEIGHT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_WEIGHT` writer - "]
pub struct ARB_WEIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_WEIGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ARB_WMB` reader - "]
pub struct ARB_WMB_R(crate::FieldReader<u8>);
impl ARB_WMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ARB_WMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_WMB_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_WMB` writer - "]
pub struct ARB_WMB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_WMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn arb_weight(&self) -> ARB_WEIGHT_R {
        ARB_WEIGHT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn arb_wmb(&self) -> ARB_WMB_R {
        ARB_WMB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn arb_weight(&mut self) -> ARB_WEIGHT_W {
        ARB_WEIGHT_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn arb_wmb(&mut self) -> ARB_WMB_W {
        ARB_WMB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WMB and Weight share same port to access external memory. This register controls the weight factor in the arbiter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_arbiter](index.html) module"]
pub struct S_ARBITER_SPEC;
impl crate::RegisterSpec for S_ARBITER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_arbiter::R](R) reader structure"]
impl crate::Readable for S_ARBITER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_arbiter::W](W) writer structure"]
impl crate::Writable for S_ARBITER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_ARBITER to value 0x3000_0000"]
impl crate::Resettable for S_ARBITER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000_0000
    }
}
