#[doc = "Register `CMAC_A_BASE_ATOMIC_C_0` reader"]
pub struct R(crate::R<CMAC_A_BASE_ATOMIC_C_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAC_A_BASE_ATOMIC_C_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAC_A_BASE_ATOMIC_C_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAC_A_BASE_ATOMIC_C_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMAC_A_BASE_ATOMIC_C` reader - "]
pub type CMAC_A_BASE_ATOMIC_C_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmac_a_base_atomic_c(&self) -> CMAC_A_BASE_ATOMIC_C_R {
        CMAC_A_BASE_ATOMIC_C_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmac_a_base_atomic_c_0](index.html) module"]
pub struct CMAC_A_BASE_ATOMIC_C_0_SPEC;
impl crate::RegisterSpec for CMAC_A_BASE_ATOMIC_C_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmac_a_base_atomic_c_0::R](R) reader structure"]
impl crate::Readable for CMAC_A_BASE_ATOMIC_C_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMAC_A_BASE_ATOMIC_C_0 to value 0x08"]
impl crate::Resettable for CMAC_A_BASE_ATOMIC_C_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
