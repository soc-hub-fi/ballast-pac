#[doc = "Register `cycle_count_low` reader"]
pub struct R(crate::R<CYCLE_COUNT_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE_COUNT_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE_COUNT_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE_COUNT_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `cycle_count_low` reader - Cycle count (low 32 bits)"]
pub struct CYCLE_COUNT_LOW_R(crate::FieldReader<u32>);
impl CYCLE_COUNT_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CYCLE_COUNT_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLE_COUNT_LOW_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Cycle count (low 32 bits)"]
    #[inline(always)]
    pub fn cycle_count_low(&self) -> CYCLE_COUNT_LOW_R {
        CYCLE_COUNT_LOW_R::new(self.bits)
    }
}
#[doc = "Low part of Cycle count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle_count_low](index.html) module"]
pub struct CYCLE_COUNT_LOW_SPEC;
impl crate::RegisterSpec for CYCLE_COUNT_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle_count_low::R](R) reader structure"]
impl crate::Readable for CYCLE_COUNT_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cycle_count_low to value 0"]
impl crate::Resettable for CYCLE_COUNT_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
