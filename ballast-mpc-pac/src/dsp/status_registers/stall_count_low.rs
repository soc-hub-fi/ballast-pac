#[doc = "Register `stall_count_low` reader"]
pub struct R(crate::R<STALL_COUNT_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_COUNT_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_COUNT_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_COUNT_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stall_count_low` reader - Stall count (low 32 bits)"]
pub type STALL_COUNT_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stall count (low 32 bits)"]
    #[inline(always)]
    pub fn stall_count_low(&self) -> STALL_COUNT_LOW_R {
        STALL_COUNT_LOW_R::new(self.bits)
    }
}
#[doc = "Low part of Stall count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_count_low](index.html) module"]
pub struct STALL_COUNT_LOW_SPEC;
impl crate::RegisterSpec for STALL_COUNT_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stall_count_low::R](R) reader structure"]
impl crate::Readable for STALL_COUNT_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets stall_count_low to value 0"]
impl crate::Resettable for STALL_COUNT_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
