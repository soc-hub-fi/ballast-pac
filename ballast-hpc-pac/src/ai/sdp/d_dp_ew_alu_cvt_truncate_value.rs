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
#[doc = "Register `D_DP_EW_ALU_CVT_TRUNCATE_VALUE` writer"]
pub struct W(crate::W<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EW_ALU_CVT_TRUNCATE` reader - "]
pub type EW_ALU_CVT_TRUNCATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EW_ALU_CVT_TRUNCATE` writer - "]
pub type EW_ALU_CVT_TRUNCATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ew_alu_cvt_truncate(&self) -> EW_ALU_CVT_TRUNCATE_R {
        EW_ALU_CVT_TRUNCATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn ew_alu_cvt_truncate(&mut self) -> EW_ALU_CVT_TRUNCATE_W<0> {
        EW_ALU_CVT_TRUNCATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Converter truncate of EW ALU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_ew_alu_cvt_truncate_value](index.html) module"]
pub struct D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC;
impl crate::RegisterSpec for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_ew_alu_cvt_truncate_value::R](R) reader structure"]
impl crate::Readable for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_ew_alu_cvt_truncate_value::W](W) writer structure"]
impl crate::Writable for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DP_EW_ALU_CVT_TRUNCATE_VALUE to value 0"]
impl crate::Resettable for D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
