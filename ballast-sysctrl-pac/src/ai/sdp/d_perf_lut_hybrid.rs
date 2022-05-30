#[doc = "Register `D_PERF_LUT_HYBRID` reader"]
pub struct R(crate::R<D_PERF_LUT_HYBRID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_LUT_HYBRID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_LUT_HYBRID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_LUT_HYBRID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUT_HYBRID` reader - "]
pub struct LUT_HYBRID_R(crate::FieldReader<u32>);
impl LUT_HYBRID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LUT_HYBRID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_HYBRID_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lut_hybrid(&self) -> LUT_HYBRID_R {
        LUT_HYBRID_R::new(self.bits)
    }
}
#[doc = "Element number of both hit, or both miss situation that element underflow one table and at the same time overflow the other.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_lut_hybrid](index.html) module"]
pub struct D_PERF_LUT_HYBRID_SPEC;
impl crate::RegisterSpec for D_PERF_LUT_HYBRID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_lut_hybrid::R](R) reader structure"]
impl crate::Readable for D_PERF_LUT_HYBRID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_LUT_HYBRID to value 0"]
impl crate::Resettable for D_PERF_LUT_HYBRID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
