#[doc = "Register `D_PERF_LUT_LO_HIT` reader"]
pub struct R(crate::R<D_PERF_LUT_LO_HIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_LUT_LO_HIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_LUT_LO_HIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_LUT_LO_HIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUT_LO_HIT` reader - "]
pub type LUT_LO_HIT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lut_lo_hit(&self) -> LUT_LO_HIT_R {
        LUT_LO_HIT_R::new(self.bits)
    }
}
#[doc = "Element number of only LO table hit\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_lut_lo_hit](index.html) module"]
pub struct D_PERF_LUT_LO_HIT_SPEC;
impl crate::RegisterSpec for D_PERF_LUT_LO_HIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_lut_lo_hit::R](R) reader structure"]
impl crate::Readable for D_PERF_LUT_LO_HIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_LUT_LO_HIT to value 0"]
impl crate::Resettable for D_PERF_LUT_LO_HIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
