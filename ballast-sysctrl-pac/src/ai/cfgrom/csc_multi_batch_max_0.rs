#[doc = "Register `CSC_MULTI_BATCH_MAX_0` reader"]
pub struct R(crate::R<CSC_MULTI_BATCH_MAX_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSC_MULTI_BATCH_MAX_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSC_MULTI_BATCH_MAX_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSC_MULTI_BATCH_MAX_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSC_MULTI_BATCH_MAX` reader - "]
pub struct CSC_MULTI_BATCH_MAX_R(crate::FieldReader<u32, u32>);
impl CSC_MULTI_BATCH_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CSC_MULTI_BATCH_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_MULTI_BATCH_MAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn csc_multi_batch_max(&self) -> CSC_MULTI_BATCH_MAX_R {
        CSC_MULTI_BATCH_MAX_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc_multi_batch_max_0](index.html) module"]
pub struct CSC_MULTI_BATCH_MAX_0_SPEC;
impl crate::RegisterSpec for CSC_MULTI_BATCH_MAX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csc_multi_batch_max_0::R](R) reader structure"]
impl crate::Readable for CSC_MULTI_BATCH_MAX_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSC_MULTI_BATCH_MAX_0 to value 0"]
impl crate::Resettable for CSC_MULTI_BATCH_MAX_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
