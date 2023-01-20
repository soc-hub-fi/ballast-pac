#[doc = "Register `CACC_BASE_ATOMIC_C_0` reader"]
pub struct R(crate::R<CACC_BASE_ATOMIC_C_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACC_BASE_ATOMIC_C_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACC_BASE_ATOMIC_C_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACC_BASE_ATOMIC_C_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACC_BASE_ATOMIC_C` reader - "]
pub type CACC_BASE_ATOMIC_C_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cacc_base_atomic_c(&self) -> CACC_BASE_ATOMIC_C_R {
        CACC_BASE_ATOMIC_C_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacc_base_atomic_c_0](index.html) module"]
pub struct CACC_BASE_ATOMIC_C_0_SPEC;
impl crate::RegisterSpec for CACC_BASE_ATOMIC_C_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacc_base_atomic_c_0::R](R) reader structure"]
impl crate::Readable for CACC_BASE_ATOMIC_C_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACC_BASE_ATOMIC_C_0 to value 0x08"]
impl crate::Resettable for CACC_BASE_ATOMIC_C_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
