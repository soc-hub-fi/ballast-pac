#[doc = "Register `D_DP_BN_CFG` reader"]
pub struct R(crate::R<D_DP_BN_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BN_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BN_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BN_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BN_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BN_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BN_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BN_BYPASS` reader - "]
pub struct BN_BYPASS_R(crate::FieldReader<bool, BN_BYPASS_A>);
impl BN_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BN_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BN_BYPASS_A {
        match self.bits {
            true => BN_BYPASS_A::YES,
            false => BN_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BN_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BN_BYPASS_A::NO
    }
}
impl core::ops::Deref for BN_BYPASS_R {
    type Target = crate::FieldReader<bool, BN_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BN_ALU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BN_ALU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BN_ALU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BN_ALU_BYPASS` reader - "]
pub struct BN_ALU_BYPASS_R(crate::FieldReader<bool, BN_ALU_BYPASS_A>);
impl BN_ALU_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BN_ALU_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BN_ALU_BYPASS_A {
        match self.bits {
            true => BN_ALU_BYPASS_A::YES,
            false => BN_ALU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BN_ALU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BN_ALU_BYPASS_A::NO
    }
}
impl core::ops::Deref for BN_ALU_BYPASS_R {
    type Target = crate::FieldReader<bool, BN_ALU_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BN_ALU_ALGO_A {
    #[doc = "2: `10`"]
    SUM = 2,
    #[doc = "1: `1`"]
    MIN = 1,
    #[doc = "0: `0`"]
    MAX = 0,
}
impl From<BN_ALU_ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: BN_ALU_ALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BN_ALU_ALGO` reader - "]
pub struct BN_ALU_ALGO_R(crate::FieldReader<u8, BN_ALU_ALGO_A>);
impl BN_ALU_ALGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BN_ALU_ALGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BN_ALU_ALGO_A> {
        match self.bits {
            2 => Some(BN_ALU_ALGO_A::SUM),
            1 => Some(BN_ALU_ALGO_A::MIN),
            0 => Some(BN_ALU_ALGO_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SUM`"]
    #[inline(always)]
    pub fn is_sum(&self) -> bool {
        **self == BN_ALU_ALGO_A::SUM
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        **self == BN_ALU_ALGO_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        **self == BN_ALU_ALGO_A::MAX
    }
}
impl core::ops::Deref for BN_ALU_ALGO_R {
    type Target = crate::FieldReader<u8, BN_ALU_ALGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BN_MUL_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BN_MUL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BN_MUL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BN_MUL_BYPASS` reader - "]
pub struct BN_MUL_BYPASS_R(crate::FieldReader<bool, BN_MUL_BYPASS_A>);
impl BN_MUL_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BN_MUL_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BN_MUL_BYPASS_A {
        match self.bits {
            true => BN_MUL_BYPASS_A::YES,
            false => BN_MUL_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BN_MUL_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BN_MUL_BYPASS_A::NO
    }
}
impl core::ops::Deref for BN_MUL_BYPASS_R {
    type Target = crate::FieldReader<bool, BN_MUL_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BN_MUL_PRELU_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BN_MUL_PRELU_A> for bool {
    #[inline(always)]
    fn from(variant: BN_MUL_PRELU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BN_MUL_PRELU` reader - "]
pub struct BN_MUL_PRELU_R(crate::FieldReader<bool, BN_MUL_PRELU_A>);
impl BN_MUL_PRELU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BN_MUL_PRELU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BN_MUL_PRELU_A {
        match self.bits {
            true => BN_MUL_PRELU_A::YES,
            false => BN_MUL_PRELU_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BN_MUL_PRELU_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BN_MUL_PRELU_A::NO
    }
}
impl core::ops::Deref for BN_MUL_PRELU_R {
    type Target = crate::FieldReader<bool, BN_MUL_PRELU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BN_RELU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BN_RELU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BN_RELU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BN_RELU_BYPASS` reader - "]
pub struct BN_RELU_BYPASS_R(crate::FieldReader<bool, BN_RELU_BYPASS_A>);
impl BN_RELU_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BN_RELU_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BN_RELU_BYPASS_A {
        match self.bits {
            true => BN_RELU_BYPASS_A::YES,
            false => BN_RELU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BN_RELU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BN_RELU_BYPASS_A::NO
    }
}
impl core::ops::Deref for BN_RELU_BYPASS_R {
    type Target = crate::FieldReader<bool, BN_RELU_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bn_bypass(&self) -> BN_BYPASS_R {
        BN_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bn_alu_bypass(&self) -> BN_ALU_BYPASS_R {
        BN_ALU_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bn_alu_algo(&self) -> BN_ALU_ALGO_R {
        BN_ALU_ALGO_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bn_mul_bypass(&self) -> BN_MUL_BYPASS_R {
        BN_MUL_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bn_mul_prelu(&self) -> BN_MUL_PRELU_R {
        BN_MUL_PRELU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bn_relu_bypass(&self) -> BN_RELU_BYPASS_R {
        BN_RELU_BYPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Configurations of BN module: bypass, algorithm, etc.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bn_cfg](index.html) module"]
pub struct D_DP_BN_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BN_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bn_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BN_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_BN_CFG to value 0x53"]
impl crate::Resettable for D_DP_BN_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x53
    }
}
