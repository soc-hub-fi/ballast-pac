#[doc = "Register `D_PERF_OUT_SATURATION` reader"]
pub struct R(crate::R<D_PERF_OUT_SATURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_OUT_SATURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_OUT_SATURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_OUT_SATURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_SATURATION` reader - "]
pub type OUT_SATURATION_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn out_saturation(&self) -> OUT_SATURATION_R {
        OUT_SATURATION_R::new(self.bits)
    }
}
#[doc = "Element number of both table saturation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_out_saturation](index.html) module"]
pub struct D_PERF_OUT_SATURATION_SPEC;
impl crate::RegisterSpec for D_PERF_OUT_SATURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_out_saturation::R](R) reader structure"]
impl crate::Readable for D_PERF_OUT_SATURATION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_OUT_SATURATION to value 0"]
impl crate::Resettable for D_PERF_OUT_SATURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
