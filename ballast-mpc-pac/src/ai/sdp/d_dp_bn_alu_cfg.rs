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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `BN_ALU_SRC` reader - "]
pub struct BN_ALU_SRC_R(crate::FieldReader<bool>);
impl BN_ALU_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BN_ALU_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == BN_ALU_SRC_A::REG
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        **self == BN_ALU_SRC_A::MEM
    }
}
impl core::ops::Deref for BN_ALU_SRC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BN_ALU_SHIFT_VALUE` reader - "]
pub struct BN_ALU_SHIFT_VALUE_R(crate::FieldReader<u8>);
impl BN_ALU_SHIFT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BN_ALU_SHIFT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BN_ALU_SHIFT_VALUE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Source type and shifter value of BN ALU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bn_alu_cfg](index.html) module"]
pub struct D_DP_BN_ALU_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BN_ALU_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bn_alu_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BN_ALU_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_BN_ALU_CFG to value 0"]
impl crate::Resettable for D_DP_BN_ALU_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
