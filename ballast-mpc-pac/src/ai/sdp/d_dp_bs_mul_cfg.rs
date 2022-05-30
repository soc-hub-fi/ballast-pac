#[doc = "Register `D_DP_BS_MUL_CFG` reader"]
pub struct R(crate::R<D_DP_BS_MUL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BS_MUL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BS_MUL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BS_MUL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_MUL_SRC_A {
    #[doc = "0: `0`"]
    REG = 0,
    #[doc = "1: `1`"]
    MEM = 1,
}
impl From<BS_MUL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: BS_MUL_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_MUL_SRC` reader - "]
pub struct BS_MUL_SRC_R(crate::FieldReader<bool>);
impl BS_MUL_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_MUL_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_MUL_SRC_A {
        match self.bits {
            false => BS_MUL_SRC_A::REG,
            true => BS_MUL_SRC_A::MEM,
        }
    }
    #[doc = "Checks if the value of the field is `REG`"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        **self == BS_MUL_SRC_A::REG
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        **self == BS_MUL_SRC_A::MEM
    }
}
impl core::ops::Deref for BS_MUL_SRC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS_MUL_SHIFT_VALUE` reader - "]
pub struct BS_MUL_SHIFT_VALUE_R(crate::FieldReader<u8>);
impl BS_MUL_SHIFT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BS_MUL_SHIFT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS_MUL_SHIFT_VALUE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bs_mul_src(&self) -> BS_MUL_SRC_R {
        BS_MUL_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bs_mul_shift_value(&self) -> BS_MUL_SHIFT_VALUE_R {
        BS_MUL_SHIFT_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Source type and shifter value of BS MUL\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bs_mul_cfg](index.html) module"]
pub struct D_DP_BS_MUL_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BS_MUL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bs_mul_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BS_MUL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_BS_MUL_CFG to value 0"]
impl crate::Resettable for D_DP_BS_MUL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
