#[doc = "Register `program_counter` reader"]
pub struct R(crate::R<PROGRAM_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROGRAM_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROGRAM_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROGRAM_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `program_counter` reader - "]
pub type PROGRAM_COUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn program_counter(&self) -> PROGRAM_COUNTER_R {
        PROGRAM_COUNTER_R::new(self.bits)
    }
}
#[doc = "Program counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [program_counter](index.html) module"]
pub struct PROGRAM_COUNTER_SPEC;
impl crate::RegisterSpec for PROGRAM_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [program_counter::R](R) reader structure"]
impl crate::Readable for PROGRAM_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets program_counter to value 0"]
impl crate::Resettable for PROGRAM_COUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
