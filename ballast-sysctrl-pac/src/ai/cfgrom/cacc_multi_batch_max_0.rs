#[doc = "Register `CACC_MULTI_BATCH_MAX_0` reader"]
pub struct R(crate::R<CACC_MULTI_BATCH_MAX_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACC_MULTI_BATCH_MAX_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACC_MULTI_BATCH_MAX_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACC_MULTI_BATCH_MAX_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACC_MULTI_BATCH_MAX` reader - "]
pub struct CACC_MULTI_BATCH_MAX_R(crate::FieldReader<u32, u32>);
impl CACC_MULTI_BATCH_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CACC_MULTI_BATCH_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACC_MULTI_BATCH_MAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cacc_multi_batch_max(&self) -> CACC_MULTI_BATCH_MAX_R {
        CACC_MULTI_BATCH_MAX_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacc_multi_batch_max_0](index.html) module"]
pub struct CACC_MULTI_BATCH_MAX_0_SPEC;
impl crate::RegisterSpec for CACC_MULTI_BATCH_MAX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacc_multi_batch_max_0::R](R) reader structure"]
impl crate::Readable for CACC_MULTI_BATCH_MAX_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACC_MULTI_BATCH_MAX_0 to value 0"]
impl crate::Resettable for CACC_MULTI_BATCH_MAX_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
