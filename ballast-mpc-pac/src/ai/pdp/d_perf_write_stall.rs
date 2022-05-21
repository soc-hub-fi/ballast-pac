#[doc = "Register `D_PERF_WRITE_STALL` reader"]
pub struct R(crate::R<D_PERF_WRITE_STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_WRITE_STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_WRITE_STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_WRITE_STALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERF_WRITE_STALL` reader - "]
pub struct PERF_WRITE_STALL_R(crate::FieldReader<u32, u32>);
impl PERF_WRITE_STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERF_WRITE_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERF_WRITE_STALL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn perf_write_stall(&self) -> PERF_WRITE_STALL_R {
        PERF_WRITE_STALL_R::new(self.bits)
    }
}
#[doc = "Counting stalls of write requests\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_write_stall](index.html) module"]
pub struct D_PERF_WRITE_STALL_SPEC;
impl crate::RegisterSpec for D_PERF_WRITE_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_write_stall::R](R) reader structure"]
impl crate::Readable for D_PERF_WRITE_STALL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_WRITE_STALL to value 0"]
impl crate::Resettable for D_PERF_WRITE_STALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
