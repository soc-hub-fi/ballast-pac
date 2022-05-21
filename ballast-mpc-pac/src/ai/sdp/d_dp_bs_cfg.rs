#[doc = "Register `D_DP_BS_CFG` reader"]
pub struct R(crate::R<D_DP_BS_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BS_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BS_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_BYPASS` reader - "]
pub struct BS_BYPASS_R(crate::FieldReader<bool, BS_BYPASS_A>);
impl BS_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_BYPASS_A {
        match self.bits {
            true => BS_BYPASS_A::YES,
            false => BS_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BS_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BS_BYPASS_A::NO
    }
}
impl core::ops::Deref for BS_BYPASS_R {
    type Target = crate::FieldReader<bool, BS_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_ALU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_ALU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_ALU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_ALU_BYPASS` reader - "]
pub struct BS_ALU_BYPASS_R(crate::FieldReader<bool, BS_ALU_BYPASS_A>);
impl BS_ALU_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_ALU_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_ALU_BYPASS_A {
        match self.bits {
            true => BS_ALU_BYPASS_A::YES,
            false => BS_ALU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BS_ALU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BS_ALU_BYPASS_A::NO
    }
}
impl core::ops::Deref for BS_ALU_BYPASS_R {
    type Target = crate::FieldReader<bool, BS_ALU_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BS_ALU_ALGO_A {
    #[doc = "0: `0`"]
    MAX = 0,
    #[doc = "1: `1`"]
    MIN = 1,
    #[doc = "2: `10`"]
    SUM = 2,
}
impl From<BS_ALU_ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: BS_ALU_ALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BS_ALU_ALGO` reader - "]
pub struct BS_ALU_ALGO_R(crate::FieldReader<u8, BS_ALU_ALGO_A>);
impl BS_ALU_ALGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BS_ALU_ALGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BS_ALU_ALGO_A> {
        match self.bits {
            0 => Some(BS_ALU_ALGO_A::MAX),
            1 => Some(BS_ALU_ALGO_A::MIN),
            2 => Some(BS_ALU_ALGO_A::SUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        **self == BS_ALU_ALGO_A::MAX
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        **self == BS_ALU_ALGO_A::MIN
    }
    #[doc = "Checks if the value of the field is `SUM`"]
    #[inline(always)]
    pub fn is_sum(&self) -> bool {
        **self == BS_ALU_ALGO_A::SUM
    }
}
impl core::ops::Deref for BS_ALU_ALGO_R {
    type Target = crate::FieldReader<u8, BS_ALU_ALGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_MUL_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_MUL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_MUL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_MUL_BYPASS` reader - "]
pub struct BS_MUL_BYPASS_R(crate::FieldReader<bool, BS_MUL_BYPASS_A>);
impl BS_MUL_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_MUL_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_MUL_BYPASS_A {
        match self.bits {
            true => BS_MUL_BYPASS_A::YES,
            false => BS_MUL_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BS_MUL_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BS_MUL_BYPASS_A::NO
    }
}
impl core::ops::Deref for BS_MUL_BYPASS_R {
    type Target = crate::FieldReader<bool, BS_MUL_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_MUL_PRELU_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_MUL_PRELU_A> for bool {
    #[inline(always)]
    fn from(variant: BS_MUL_PRELU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_MUL_PRELU` reader - "]
pub struct BS_MUL_PRELU_R(crate::FieldReader<bool, BS_MUL_PRELU_A>);
impl BS_MUL_PRELU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_MUL_PRELU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_MUL_PRELU_A {
        match self.bits {
            true => BS_MUL_PRELU_A::YES,
            false => BS_MUL_PRELU_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BS_MUL_PRELU_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BS_MUL_PRELU_A::NO
    }
}
impl core::ops::Deref for BS_MUL_PRELU_R {
    type Target = crate::FieldReader<bool, BS_MUL_PRELU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_RELU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_RELU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_RELU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_RELU_BYPASS` reader - "]
pub struct BS_RELU_BYPASS_R(crate::FieldReader<bool, BS_RELU_BYPASS_A>);
impl BS_RELU_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_RELU_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_RELU_BYPASS_A {
        match self.bits {
            true => BS_RELU_BYPASS_A::YES,
            false => BS_RELU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BS_RELU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BS_RELU_BYPASS_A::NO
    }
}
impl core::ops::Deref for BS_RELU_BYPASS_R {
    type Target = crate::FieldReader<bool, BS_RELU_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bs_bypass(&self) -> BS_BYPASS_R {
        BS_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bs_alu_bypass(&self) -> BS_ALU_BYPASS_R {
        BS_ALU_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bs_alu_algo(&self) -> BS_ALU_ALGO_R {
        BS_ALU_ALGO_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bs_mul_bypass(&self) -> BS_MUL_BYPASS_R {
        BS_MUL_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bs_mul_prelu(&self) -> BS_MUL_PRELU_R {
        BS_MUL_PRELU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bs_relu_bypass(&self) -> BS_RELU_BYPASS_R {
        BS_RELU_BYPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Configurations of BS module: bypass, algorithm, etc.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bs_cfg](index.html) module"]
pub struct D_DP_BS_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bs_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BS_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_BS_CFG to value 0x73"]
impl crate::Resettable for D_DP_BS_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x73
    }
}
