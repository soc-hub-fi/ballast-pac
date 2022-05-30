#[doc = "Register `D_PERF_LUT_OFLOW` reader"]
pub struct R(crate::R<D_PERF_LUT_OFLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_LUT_OFLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_LUT_OFLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_LUT_OFLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUT_OFLOW` reader - "]
pub struct LUT_OFLOW_R(crate::FieldReader<u32>);
impl LUT_OFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LUT_OFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_OFLOW_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lut_oflow(&self) -> LUT_OFLOW_R {
        LUT_OFLOW_R::new(self.bits)
    }
}
#[doc = "Element number of both table overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_lut_oflow](index.html) module"]
pub struct D_PERF_LUT_OFLOW_SPEC;
impl crate::RegisterSpec for D_PERF_LUT_OFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_lut_oflow::R](R) reader structure"]
impl crate::Readable for D_PERF_LUT_OFLOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_LUT_OFLOW to value 0"]
impl crate::Resettable for D_PERF_LUT_OFLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
