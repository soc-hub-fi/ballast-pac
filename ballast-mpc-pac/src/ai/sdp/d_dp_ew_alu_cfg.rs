#[doc = "Register `D_DP_EW_ALU_CFG` reader"]
pub struct R(crate::R<D_DP_EW_ALU_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_EW_ALU_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_EW_ALU_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_EW_ALU_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_ALU_SRC_A {
    #[doc = "0: `0`"]
    REG = 0,
    #[doc = "1: `1`"]
    MEM = 1,
}
impl From<EW_ALU_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: EW_ALU_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_ALU_SRC` reader - "]
pub struct EW_ALU_SRC_R(crate::FieldReader<bool>);
impl EW_ALU_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_ALU_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_ALU_SRC_A {
        match self.bits {
            false => EW_ALU_SRC_A::REG,
            true => EW_ALU_SRC_A::MEM,
        }
    }
    #[doc = "Checks if the value of the field is `REG`"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        **self == EW_ALU_SRC_A::REG
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        **self == EW_ALU_SRC_A::MEM
    }
}
impl core::ops::Deref for EW_ALU_SRC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_ALU_CVT_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_ALU_CVT_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EW_ALU_CVT_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW_ALU_CVT_BYPASS` reader - "]
pub struct EW_ALU_CVT_BYPASS_R(crate::FieldReader<bool>);
impl EW_ALU_CVT_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_ALU_CVT_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_ALU_CVT_BYPASS_A {
        match self.bits {
            true => EW_ALU_CVT_BYPASS_A::YES,
            false => EW_ALU_CVT_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == EW_ALU_CVT_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == EW_ALU_CVT_BYPASS_A::NO
    }
}
impl core::ops::Deref for EW_ALU_CVT_BYPASS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ew_alu_src(&self) -> EW_ALU_SRC_R {
        EW_ALU_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ew_alu_cvt_bypass(&self) -> EW_ALU_CVT_BYPASS_R {
        EW_ALU_CVT_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Source type and bypass control of EW ALU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_ew_alu_cfg](index.html) module"]
pub struct D_DP_EW_ALU_CFG_SPEC;
impl crate::RegisterSpec for D_DP_EW_ALU_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_ew_alu_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_EW_ALU_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DP_EW_ALU_CFG to value 0x02"]
impl crate::Resettable for D_DP_EW_ALU_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
