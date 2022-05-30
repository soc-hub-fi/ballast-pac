#[doc = "Register `D_DP_EW_ALU_CVT_TRUNCATE_VALUE` reader"]
pub struct R(crate::R<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EW_ALU_CVT_TRUNCATE` reader - "]
pub struct EW_ALU_CVT_TRUNCATE_R(crate::FieldReader<u8>);
impl EW_ALU_CVT_TRUNCATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EW_ALU_CVT_TRUNCATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_ALU_CVT_TRUNCATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ew_alu_cvt_truncate(&self) -> EW_ALU_CVT_TRUNCATE_R {
        EW_ALU_CVT_TRUNCATE_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Converter truncate of EW ALU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_ew_alu_cvt_truncate_value](index.html) module"]
pub struct D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC;
impl crate::RegisterSpec for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_ew_alu_cvt_truncate_value::R](R) reader structure"]
impl crate::Readable for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_EW_ALU_CVT_TRUNCATE_VALUE to value 0"]
impl crate::Resettable for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
