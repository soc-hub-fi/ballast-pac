#[doc = "Register `D_DP_BN_MUL_SRC_VALUE` reader"]
pub struct R(crate::R<D_DP_BN_MUL_SRC_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BN_MUL_SRC_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BN_MUL_SRC_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BN_MUL_SRC_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DP_BN_MUL_SRC_VALUE` writer"]
pub struct W(crate::W<D_DP_BN_MUL_SRC_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_BN_MUL_SRC_VALUE_SPEC>;
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
impl From<crate::W<D_DP_BN_MUL_SRC_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_BN_MUL_SRC_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BN_MUL_OPERAND` reader - "]
pub struct BN_MUL_OPERAND_R(crate::FieldReader<u16>);
impl BN_MUL_OPERAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BN_MUL_OPERAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BN_MUL_OPERAND_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BN_MUL_OPERAND` writer - "]
pub struct BN_MUL_OPERAND_W<'a> {
    w: &'a mut W,
}
impl<'a> BN_MUL_OPERAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bn_mul_operand(&self) -> BN_MUL_OPERAND_R {
        BN_MUL_OPERAND_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bn_mul_operand(&mut self) -> BN_MUL_OPERAND_W {
        BN_MUL_OPERAND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operand value of BN MUL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bn_mul_src_value](index.html) module"]
pub struct D_DP_BN_MUL_SRC_VALUE_SPEC;
impl crate::RegisterSpec for D_DP_BN_MUL_SRC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bn_mul_src_value::R](R) reader structure"]
impl crate::Readable for D_DP_BN_MUL_SRC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_bn_mul_src_value::W](W) writer structure"]
impl crate::Writable for D_DP_BN_MUL_SRC_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DP_BN_MUL_SRC_VALUE to value 0"]
impl crate::Resettable for D_DP_BN_MUL_SRC_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
