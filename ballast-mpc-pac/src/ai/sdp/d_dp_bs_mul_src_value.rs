#[doc = "Register `D_DP_BS_MUL_SRC_VALUE` reader"]
pub struct R(crate::R<D_DP_BS_MUL_SRC_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BS_MUL_SRC_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BS_MUL_SRC_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BS_MUL_SRC_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BS_MUL_OPERAND` reader - "]
pub struct BS_MUL_OPERAND_R(crate::FieldReader<u16, u16>);
impl BS_MUL_OPERAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BS_MUL_OPERAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS_MUL_OPERAND_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bs_mul_operand(&self) -> BS_MUL_OPERAND_R {
        BS_MUL_OPERAND_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Operand value of BS MUL\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bs_mul_src_value](index.html) module"]
pub struct D_DP_BS_MUL_SRC_VALUE_SPEC;
impl crate::RegisterSpec for D_DP_BS_MUL_SRC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bs_mul_src_value::R](R) reader structure"]
impl crate::Readable for D_DP_BS_MUL_SRC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_BS_MUL_SRC_VALUE to value 0"]
impl crate::Resettable for D_DP_BS_MUL_SRC_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
