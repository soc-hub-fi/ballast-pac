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
#[doc = "WMB and Weight share same port to access external memory. This register controls the weight factor in the arbiter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_arbiter](index.html) module"]
pub struct S_ARBITER_SPEC;
impl crate::RegisterSpec for S_ARBITER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_arbiter::R](R) reader structure"]
impl crate::Readable for S_ARBITER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_ARBITER to value 0x3000_0000"]
impl crate::Resettable for S_ARBITER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000_0000
    }
}
