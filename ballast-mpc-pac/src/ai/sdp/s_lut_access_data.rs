#[doc = "Register `S_LUT_ACCESS_DATA` reader"]
pub struct R(crate::R<S_LUT_ACCESS_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_ACCESS_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_ACCESS_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_ACCESS_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUT_DATA` reader - "]
pub struct LUT_DATA_R(crate::FieldReader<u16, u16>);
impl LUT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LUT_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_data(&self) -> LUT_DATA_R {
        LUT_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Data register of read or write LUT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_access_data](index.html) module"]
pub struct S_LUT_ACCESS_DATA_SPEC;
impl crate::RegisterSpec for S_LUT_ACCESS_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_access_data::R](R) reader structure"]
impl crate::Readable for S_LUT_ACCESS_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_LUT_ACCESS_DATA to value 0"]
impl crate::Resettable for S_LUT_ACCESS_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
