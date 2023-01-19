#[doc = "Register `D_DP_BN_ALU_CFG` reader"]
pub struct R(crate::R<D_DP_BN_ALU_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BN_ALU_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BN_ALU_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BN_ALU_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DP_BN_ALU_CFG` writer"]
pub struct W(crate::W<D_DP_BN_ALU_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_BN_ALU_CFG_SPEC>;
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
impl From<crate::W<D_DP_BN_ALU_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_BN_ALU_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BN_ALU_SRC` reader - "]
pub type BN_ALU_SRC_R = crate::BitReader<BN_ALU_SRC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BN_ALU_SRC_A {
    #[doc = "0: `0`"]
    REG = 0,
    #[doc = "1: `1`"]
    MEM = 1,
}
impl From<BN_ALU_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: BN_ALU_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl BN_ALU_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BN_ALU_SRC_A {
        match self.bits {
            false => BN_ALU_SRC_A::REG,
            true => BN_ALU_SRC_A::MEM,
        }
    }
    #[doc = "Checks if the value of the field is `REG`"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        *self == BN_ALU_SRC_A::REG
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        *self == BN_ALU_SRC_A::MEM
    }
}
#[doc = "Field `BN_ALU_SRC` writer - "]
pub type BN_ALU_SRC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_BN_ALU_CFG_SPEC, BN_ALU_SRC_A, O>;
impl<'a, const O: u8> BN_ALU_SRC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reg(self) -> &'a mut W {
        self.variant(BN_ALU_SRC_A::REG)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mem(self) -> &'a mut W {
        self.variant(BN_ALU_SRC_A::MEM)
    }
}
#[doc = "Field `BN_ALU_SHIFT_VALUE` reader - "]
pub type BN_ALU_SHIFT_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BN_ALU_SHIFT_VALUE` writer - "]
pub type BN_ALU_SHIFT_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DP_BN_ALU_CFG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bn_alu_src(&self) -> BN_ALU_SRC_R {
        BN_ALU_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn bn_alu_shift_value(&self) -> BN_ALU_SHIFT_VALUE_R {
        BN_ALU_SHIFT_VALUE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bn_alu_src(&mut self) -> BN_ALU_SRC_W<0> {
        BN_ALU_SRC_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn bn_alu_shift_value(&mut self) -> BN_ALU_SHIFT_VALUE_W<8> {
        BN_ALU_SHIFT_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source type and shifter value of BN ALU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bn_alu_cfg](index.html) module"]
pub struct D_DP_BN_ALU_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BN_ALU_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bn_alu_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BN_ALU_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_bn_alu_cfg::W](W) writer structure"]
impl crate::Writable for D_DP_BN_ALU_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DP_BN_ALU_CFG to value 0"]
impl crate::Resettable for D_DP_BN_ALU_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
