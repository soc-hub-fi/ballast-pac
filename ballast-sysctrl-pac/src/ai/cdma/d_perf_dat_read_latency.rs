#[doc = "Register `D_PERF_DAT_READ_LATENCY` reader"]
pub struct R(crate::R<D_PERF_DAT_READ_LATENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_DAT_READ_LATENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_DAT_READ_LATENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_DAT_READ_LATENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAT_RD_LATENCY` reader - "]
pub struct DAT_RD_LATENCY_R(crate::FieldReader<u32>);
impl DAT_RD_LATENCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DAT_RD_LATENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_RD_LATENCY_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dat_rd_latency(&self) -> DAT_RD_LATENCY_R {
        DAT_RD_LATENCY_R::new(self.bits)
    }
}
#[doc = "Count total latency cycles of read response of input data, update per layer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_dat_read_latency](index.html) module"]
pub struct D_PERF_DAT_READ_LATENCY_SPEC;
impl crate::RegisterSpec for D_PERF_DAT_READ_LATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_dat_read_latency::R](R) reader structure"]
impl crate::Readable for D_PERF_DAT_READ_LATENCY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_DAT_READ_LATENCY to value 0"]
impl crate::Resettable for D_PERF_DAT_READ_LATENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
