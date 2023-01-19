#[doc = "Register `CORE_STATUS_1` reader"]
pub struct R(crate::R<CORE_STATUS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_STATUS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_STATUS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_STATUS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_STATUS_1` reader - "]
pub type CORE_STATUS_1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_1(&self) -> CORE_STATUS_1_R {
        CORE_STATUS_1_R::new(self.bits)
    }
}
#[doc = "2 Core Status registers contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_status_1](index.html) module"]
pub struct CORE_STATUS_1_SPEC;
impl crate::RegisterSpec for CORE_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_status_1::R](R) reader structure"]
impl crate::Readable for CORE_STATUS_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_STATUS_1 to value 0x01"]
impl crate::Resettable for CORE_STATUS_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
