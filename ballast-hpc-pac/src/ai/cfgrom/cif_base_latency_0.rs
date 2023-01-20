#[doc = "Register `CIF_BASE_LATENCY_0` reader"]
pub struct R(crate::R<CIF_BASE_LATENCY_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIF_BASE_LATENCY_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIF_BASE_LATENCY_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIF_BASE_LATENCY_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIF_BASE_LATENCY` reader - "]
pub type CIF_BASE_LATENCY_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cif_base_latency(&self) -> CIF_BASE_LATENCY_R {
        CIF_BASE_LATENCY_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cif_base_latency_0](index.html) module"]
pub struct CIF_BASE_LATENCY_0_SPEC;
impl crate::RegisterSpec for CIF_BASE_LATENCY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cif_base_latency_0::R](R) reader structure"]
impl crate::Readable for CIF_BASE_LATENCY_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIF_BASE_LATENCY_0 to value 0x32"]
impl crate::Resettable for CIF_BASE_LATENCY_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x32;
}
