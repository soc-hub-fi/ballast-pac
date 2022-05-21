#[doc = "Register `D_OUT_SATURATION` reader"]
pub struct R(crate::R<D_OUT_SATURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_OUT_SATURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_OUT_SATURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_OUT_SATURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAT_COUNT` reader - "]
pub struct SAT_COUNT_R(crate::FieldReader<u32, u32>);
impl SAT_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SAT_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAT_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sat_count(&self) -> SAT_COUNT_R {
        SAT_COUNT_R::new(self.bits)
    }
}
#[doc = "Output saturation count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_out_saturation](index.html) module"]
pub struct D_OUT_SATURATION_SPEC;
impl crate::RegisterSpec for D_OUT_SATURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_out_saturation::R](R) reader structure"]
impl crate::Readable for D_OUT_SATURATION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_OUT_SATURATION to value 0"]
impl crate::Resettable for D_OUT_SATURATION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
