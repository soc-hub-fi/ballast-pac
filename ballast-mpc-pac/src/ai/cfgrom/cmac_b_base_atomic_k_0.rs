#[doc = "Register `CMAC_B_BASE_ATOMIC_K_0` reader"]
pub struct R(crate::R<CMAC_B_BASE_ATOMIC_K_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAC_B_BASE_ATOMIC_K_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAC_B_BASE_ATOMIC_K_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAC_B_BASE_ATOMIC_K_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMAC_B_BASE_ATOMIC_K` reader - "]
pub struct CMAC_B_BASE_ATOMIC_K_R(crate::FieldReader<u32>);
impl CMAC_B_BASE_ATOMIC_K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMAC_B_BASE_ATOMIC_K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMAC_B_BASE_ATOMIC_K_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmac_b_base_atomic_k(&self) -> CMAC_B_BASE_ATOMIC_K_R {
        CMAC_B_BASE_ATOMIC_K_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmac_b_base_atomic_k_0](index.html) module"]
pub struct CMAC_B_BASE_ATOMIC_K_0_SPEC;
impl crate::RegisterSpec for CMAC_B_BASE_ATOMIC_K_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmac_b_base_atomic_k_0::R](R) reader structure"]
impl crate::Readable for CMAC_B_BASE_ATOMIC_K_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMAC_B_BASE_ATOMIC_K_0 to value 0x08"]
impl crate::Resettable for CMAC_B_BASE_ATOMIC_K_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
