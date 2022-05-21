#[doc = "Register `D_DP_EW_MUL_CVT_OFFSET_VALUE` reader"]
pub struct R(crate::R<D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EW_MUL_CVT_OFFSET` reader - "]
pub struct EW_MUL_CVT_OFFSET_R(crate::FieldReader<u32, u32>);
impl EW_MUL_CVT_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EW_MUL_CVT_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_MUL_CVT_OFFSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ew_mul_cvt_offset(&self) -> EW_MUL_CVT_OFFSET_R {
        EW_MUL_CVT_OFFSET_R::new(self.bits)
    }
}
#[doc = "Converter offset of EW MUL\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_ew_mul_cvt_offset_value](index.html) module"]
pub struct D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC;
impl crate::RegisterSpec for D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_ew_mul_cvt_offset_value::R](R) reader structure"]
impl crate::Readable for D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_EW_MUL_CVT_OFFSET_VALUE to value 0"]
impl crate::Resettable for D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
