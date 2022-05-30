#[doc = "Register `stall_count_high` reader"]
pub struct R(crate::R<STALL_COUNT_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_COUNT_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_COUNT_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_COUNT_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stall_count_high` reader - "]
pub struct STALL_COUNT_HIGH_R(crate::FieldReader<u32>);
impl STALL_COUNT_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STALL_COUNT_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_COUNT_HIGH_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn stall_count_high(&self) -> STALL_COUNT_HIGH_R {
        STALL_COUNT_HIGH_R::new(self.bits)
    }
}
#[doc = "High part of Stall count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_count_high](index.html) module"]
pub struct STALL_COUNT_HIGH_SPEC;
impl crate::RegisterSpec for STALL_COUNT_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stall_count_high::R](R) reader structure"]
impl crate::Readable for STALL_COUNT_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets stall_count_high to value 0"]
impl crate::Resettable for STALL_COUNT_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
