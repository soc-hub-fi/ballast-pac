#[doc = "Register `S_LUT_CFG` reader"]
pub struct R(crate::R<S_LUT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUT_LE_FUNCTION_A {
    #[doc = "0: `0`"]
    EXPONENT = 0,
    #[doc = "1: `1`"]
    LINEAR = 1,
}
impl From<LUT_LE_FUNCTION_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_LE_FUNCTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUT_LE_FUNCTION` reader - "]
pub struct LUT_LE_FUNCTION_R(crate::FieldReader<bool, LUT_LE_FUNCTION_A>);
impl LUT_LE_FUNCTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUT_LE_FUNCTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_LE_FUNCTION_A {
        match self.bits {
            false => LUT_LE_FUNCTION_A::EXPONENT,
            true => LUT_LE_FUNCTION_A::LINEAR,
        }
    }
    #[doc = "Checks if the value of the field is `EXPONENT`"]
    #[inline(always)]
    pub fn is_exponent(&self) -> bool {
        **self == LUT_LE_FUNCTION_A::EXPONENT
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        **self == LUT_LE_FUNCTION_A::LINEAR
    }
}
impl core::ops::Deref for LUT_LE_FUNCTION_R {
    type Target = crate::FieldReader<bool, LUT_LE_FUNCTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUT_UFLOW_PRIORITY_A {
    #[doc = "1: `1`"]
    LO = 1,
    #[doc = "0: `0`"]
    LE = 0,
}
impl From<LUT_UFLOW_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_UFLOW_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUT_UFLOW_PRIORITY` reader - "]
pub struct LUT_UFLOW_PRIORITY_R(crate::FieldReader<bool, LUT_UFLOW_PRIORITY_A>);
impl LUT_UFLOW_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUT_UFLOW_PRIORITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_UFLOW_PRIORITY_A {
        match self.bits {
            true => LUT_UFLOW_PRIORITY_A::LO,
            false => LUT_UFLOW_PRIORITY_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == LUT_UFLOW_PRIORITY_A::LO
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        **self == LUT_UFLOW_PRIORITY_A::LE
    }
}
impl core::ops::Deref for LUT_UFLOW_PRIORITY_R {
    type Target = crate::FieldReader<bool, LUT_UFLOW_PRIORITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUT_OFLOW_PRIORITY_A {
    #[doc = "1: `1`"]
    LO = 1,
    #[doc = "0: `0`"]
    LE = 0,
}
impl From<LUT_OFLOW_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_OFLOW_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUT_OFLOW_PRIORITY` reader - "]
pub struct LUT_OFLOW_PRIORITY_R(crate::FieldReader<bool, LUT_OFLOW_PRIORITY_A>);
impl LUT_OFLOW_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUT_OFLOW_PRIORITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_OFLOW_PRIORITY_A {
        match self.bits {
            true => LUT_OFLOW_PRIORITY_A::LO,
            false => LUT_OFLOW_PRIORITY_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == LUT_OFLOW_PRIORITY_A::LO
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        **self == LUT_OFLOW_PRIORITY_A::LE
    }
}
impl core::ops::Deref for LUT_OFLOW_PRIORITY_R {
    type Target = crate::FieldReader<bool, LUT_OFLOW_PRIORITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUT_HYBRID_PRIORITY_A {
    #[doc = "1: `1`"]
    LO = 1,
    #[doc = "0: `0`"]
    LE = 0,
}
impl From<LUT_HYBRID_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_HYBRID_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUT_HYBRID_PRIORITY` reader - "]
pub struct LUT_HYBRID_PRIORITY_R(crate::FieldReader<bool, LUT_HYBRID_PRIORITY_A>);
impl LUT_HYBRID_PRIORITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LUT_HYBRID_PRIORITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_HYBRID_PRIORITY_A {
        match self.bits {
            true => LUT_HYBRID_PRIORITY_A::LO,
            false => LUT_HYBRID_PRIORITY_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == LUT_HYBRID_PRIORITY_A::LO
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        **self == LUT_HYBRID_PRIORITY_A::LE
    }
}
impl core::ops::Deref for LUT_HYBRID_PRIORITY_R {
    type Target = crate::FieldReader<bool, LUT_HYBRID_PRIORITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lut_le_function(&self) -> LUT_LE_FUNCTION_R {
        LUT_LE_FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lut_uflow_priority(&self) -> LUT_UFLOW_PRIORITY_R {
        LUT_UFLOW_PRIORITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lut_oflow_priority(&self) -> LUT_OFLOW_PRIORITY_R {
        LUT_OFLOW_PRIORITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lut_hybrid_priority(&self) -> LUT_HYBRID_PRIORITY_R {
        LUT_HYBRID_PRIORITY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "LUT’s type: exponent or linear. And the selection between LE and LO tables.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_cfg](index.html) module"]
pub struct S_LUT_CFG_SPEC;
impl crate::RegisterSpec for S_LUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_cfg::R](R) reader structure"]
impl crate::Readable for S_LUT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_LUT_CFG to value 0"]
impl crate::Resettable for S_LUT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
