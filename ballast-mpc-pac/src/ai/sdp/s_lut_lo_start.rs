#[doc = "Register `S_LUT_LO_START` reader"]
pub struct R(crate::R<S_LUT_LO_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LO_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LO_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LO_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUT_LO_START` reader - "]
pub struct LUT_LO_START_R(crate::FieldReader<u32, u32>);
impl LUT_LO_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LUT_LO_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LO_START_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lut_lo_start(&self) -> LUT_LO_START_R {
        LUT_LO_START_R::new(self.bits)
    }
}
#[doc = "Start of LO LUT’s range\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_lo_start](index.html) module"]
pub struct S_LUT_LO_START_SPEC;
impl crate::RegisterSpec for S_LUT_LO_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_lo_start::R](R) reader structure"]
impl crate::Readable for S_LUT_LO_START_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_LUT_LO_START to value 0"]
impl crate::Resettable for S_LUT_LO_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
