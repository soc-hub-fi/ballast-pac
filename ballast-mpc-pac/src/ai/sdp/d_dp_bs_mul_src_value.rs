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
#[doc = "Register `D_DP_BS_MUL_SRC_VALUE` writer"]
pub struct W(crate::W<D_DP_BS_MUL_SRC_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_BS_MUL_SRC_VALUE_SPEC>;
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
impl From<crate::W<D_DP_BS_MUL_SRC_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_BS_MUL_SRC_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS_MUL_OPERAND` reader - "]
pub type BS_MUL_OPERAND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BS_MUL_OPERAND` writer - "]
pub type BS_MUL_OPERAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DP_BS_MUL_SRC_VALUE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bs_mul_operand(&self) -> BS_MUL_OPERAND_R {
        BS_MUL_OPERAND_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn bs_mul_operand(&mut self) -> BS_MUL_OPERAND_W<0> {
        BS_MUL_OPERAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operand value of BS MUL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bs_mul_src_value](index.html) module"]
pub struct D_DP_BS_MUL_SRC_VALUE_SPEC;
impl crate::RegisterSpec for D_DP_BS_MUL_SRC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bs_mul_src_value::R](R) reader structure"]
impl crate::Readable for D_DP_BS_MUL_SRC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_bs_mul_src_value::W](W) writer structure"]
impl crate::Writable for D_DP_BS_MUL_SRC_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DP_BS_MUL_SRC_VALUE to value 0"]
impl crate::Resettable for D_DP_BS_MUL_SRC_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
