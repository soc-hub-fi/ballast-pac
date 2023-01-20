#[doc = "Register `core_count` reader"]
pub struct R(crate::R<CORE_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `core_count` reader - "]
pub type CORE_COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_count(&self) -> CORE_COUNT_R {
        CORE_COUNT_R::new(self.bits)
    }
}
#[doc = "Core count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_count](index.html) module"]
pub struct CORE_COUNT_SPEC;
impl crate::RegisterSpec for CORE_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_count::R](R) reader structure"]
impl crate::Readable for CORE_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets core_count to value 0"]
impl crate::Resettable for CORE_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
