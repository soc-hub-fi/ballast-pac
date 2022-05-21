#[doc = "Register `D_DP_EW_CFG` reader"]
pub struct R(crate::R<D_DP_EW_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_EW_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_EW_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_EW_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EW_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_BYPASS` reader - "]
pub struct EW_BYPASS_R(crate::FieldReader<bool, EW_BYPASS_A>);
impl EW_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_BYPASS_A {
        match self.bits {
            true => EW_BYPASS_A::YES,
            false => EW_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == EW_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == EW_BYPASS_A::NO
    }
}
impl core::ops::Deref for EW_BYPASS_R {
    type Target = crate::FieldReader<bool, EW_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_ALU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_ALU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EW_ALU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_ALU_BYPASS` reader - "]
pub struct EW_ALU_BYPASS_R(crate::FieldReader<bool, EW_ALU_BYPASS_A>);
impl EW_ALU_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_ALU_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_ALU_BYPASS_A {
        match self.bits {
            true => EW_ALU_BYPASS_A::YES,
            false => EW_ALU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == EW_ALU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == EW_ALU_BYPASS_A::NO
    }
}
impl core::ops::Deref for EW_ALU_BYPASS_R {
    type Target = crate::FieldReader<bool, EW_ALU_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EW_ALU_ALGO_A {
    #[doc = "2: `10`"]
    SUM = 2,
    #[doc = "1: `1`"]
    MIN = 1,
    #[doc = "0: `0`"]
    MAX = 0,
}
impl From<EW_ALU_ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: EW_ALU_ALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EW_ALU_ALGO` reader - "]
pub struct EW_ALU_ALGO_R(crate::FieldReader<u8, EW_ALU_ALGO_A>);
impl EW_ALU_ALGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EW_ALU_ALGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EW_ALU_ALGO_A> {
        match self.bits {
            2 => Some(EW_ALU_ALGO_A::SUM),
            1 => Some(EW_ALU_ALGO_A::MIN),
            0 => Some(EW_ALU_ALGO_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SUM`"]
    #[inline(always)]
    pub fn is_sum(&self) -> bool {
        **self == EW_ALU_ALGO_A::SUM
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        **self == EW_ALU_ALGO_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        **self == EW_ALU_ALGO_A::MAX
    }
}
impl core::ops::Deref for EW_ALU_ALGO_R {
    type Target = crate::FieldReader<u8, EW_ALU_ALGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_MUL_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_MUL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EW_MUL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_MUL_BYPASS` reader - "]
pub struct EW_MUL_BYPASS_R(crate::FieldReader<bool, EW_MUL_BYPASS_A>);
impl EW_MUL_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_MUL_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_MUL_BYPASS_A {
        match self.bits {
            true => EW_MUL_BYPASS_A::YES,
            false => EW_MUL_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == EW_MUL_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == EW_MUL_BYPASS_A::NO
    }
}
impl core::ops::Deref for EW_MUL_BYPASS_R {
    type Target = crate::FieldReader<bool, EW_MUL_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_MUL_PRELU_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_MUL_PRELU_A> for bool {
    #[inline(always)]
    fn from(variant: EW_MUL_PRELU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_MUL_PRELU` reader - "]
pub struct EW_MUL_PRELU_R(crate::FieldReader<bool, EW_MUL_PRELU_A>);
impl EW_MUL_PRELU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_MUL_PRELU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_MUL_PRELU_A {
        match self.bits {
            true => EW_MUL_PRELU_A::YES,
            false => EW_MUL_PRELU_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == EW_MUL_PRELU_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == EW_MUL_PRELU_A::NO
    }
}
impl core::ops::Deref for EW_MUL_PRELU_R {
    type Target = crate::FieldReader<bool, EW_MUL_PRELU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_LUT_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_LUT_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EW_LUT_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_LUT_BYPASS` reader - "]
pub struct EW_LUT_BYPASS_R(crate::FieldReader<bool, EW_LUT_BYPASS_A>);
impl EW_LUT_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_LUT_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_LUT_BYPASS_A {
        match self.bits {
            true => EW_LUT_BYPASS_A::YES,
            false => EW_LUT_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == EW_LUT_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == EW_LUT_BYPASS_A::NO
    }
}
impl core::ops::Deref for EW_LUT_BYPASS_R {
    type Target = crate::FieldReader<bool, EW_LUT_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ew_bypass(&self) -> EW_BYPASS_R {
        EW_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ew_alu_bypass(&self) -> EW_ALU_BYPASS_R {
        EW_ALU_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ew_alu_algo(&self) -> EW_ALU_ALGO_R {
        EW_ALU_ALGO_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ew_mul_bypass(&self) -> EW_MUL_BYPASS_R {
        EW_MUL_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ew_mul_prelu(&self) -> EW_MUL_PRELU_R {
        EW_MUL_PRELU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ew_lut_bypass(&self) -> EW_LUT_BYPASS_R {
        EW_LUT_BYPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Configurations of EW module: bypass, algorithm, etc.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_ew_cfg](index.html) module"]
pub struct D_DP_EW_CFG_SPEC;
impl crate::RegisterSpec for D_DP_EW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_ew_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_EW_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_EW_CFG to value 0x53"]
impl crate::Resettable for D_DP_EW_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x53
    }
}
