#[doc = "Register `D_FEATURE_MODE_CFG` reader"]
pub struct R(crate::R<D_FEATURE_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_FEATURE_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_FEATURE_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_FEATURE_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLYING_MODE_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<FLYING_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLYING_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLYING_MODE` reader - "]
pub struct FLYING_MODE_R(crate::FieldReader<bool, FLYING_MODE_A>);
impl FLYING_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLYING_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLYING_MODE_A {
        match self.bits {
            false => FLYING_MODE_A::OFF,
            true => FLYING_MODE_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == FLYING_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == FLYING_MODE_A::ON
    }
}
impl core::ops::Deref for FLYING_MODE_R {
    type Target = crate::FieldReader<bool, FLYING_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINOGRAD_A {
    #[doc = "1: `1`"]
    ON = 1,
    #[doc = "0: `0`"]
    OFF = 0,
}
impl From<WINOGRAD_A> for bool {
    #[inline(always)]
    fn from(variant: WINOGRAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINOGRAD` reader - "]
pub struct WINOGRAD_R(crate::FieldReader<bool, WINOGRAD_A>);
impl WINOGRAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINOGRAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINOGRAD_A {
        match self.bits {
            true => WINOGRAD_A::ON,
            false => WINOGRAD_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == WINOGRAD_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == WINOGRAD_A::OFF
    }
}
impl core::ops::Deref for WINOGRAD_R {
    type Target = crate::FieldReader<bool, WINOGRAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<IN_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: IN_PRECISION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IN_PRECISION` reader - "]
pub struct IN_PRECISION_R(crate::FieldReader<u8, IN_PRECISION_A>);
impl IN_PRECISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IN_PRECISION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IN_PRECISION_A> {
        match self.bits {
            2 => Some(IN_PRECISION_A::FP16),
            1 => Some(IN_PRECISION_A::INT16),
            0 => Some(IN_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        **self == IN_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        **self == IN_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        **self == IN_PRECISION_A::INT8
    }
}
impl core::ops::Deref for IN_PRECISION_R {
    type Target = crate::FieldReader<u8, IN_PRECISION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROC_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<PROC_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: PROC_PRECISION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PROC_PRECISION` reader - "]
pub struct PROC_PRECISION_R(crate::FieldReader<u8, PROC_PRECISION_A>);
impl PROC_PRECISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROC_PRECISION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROC_PRECISION_A> {
        match self.bits {
            2 => Some(PROC_PRECISION_A::FP16),
            1 => Some(PROC_PRECISION_A::INT16),
            0 => Some(PROC_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        **self == PROC_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        **self == PROC_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        **self == PROC_PRECISION_A::INT8
    }
}
impl core::ops::Deref for PROC_PRECISION_R {
    type Target = crate::FieldReader<u8, PROC_PRECISION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<OUT_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_PRECISION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUT_PRECISION` reader - "]
pub struct OUT_PRECISION_R(crate::FieldReader<u8, OUT_PRECISION_A>);
impl OUT_PRECISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_PRECISION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_PRECISION_A> {
        match self.bits {
            2 => Some(OUT_PRECISION_A::FP16),
            1 => Some(OUT_PRECISION_A::INT16),
            0 => Some(OUT_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        **self == OUT_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        **self == OUT_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        **self == OUT_PRECISION_A::INT8
    }
}
impl core::ops::Deref for OUT_PRECISION_R {
    type Target = crate::FieldReader<u8, OUT_PRECISION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BATCH_NUMBER` reader - "]
pub struct BATCH_NUMBER_R(crate::FieldReader<u8, u8>);
impl BATCH_NUMBER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BATCH_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BATCH_NUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FLYING_MODE_R {
        FLYING_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn winograd(&self) -> WINOGRAD_R {
        WINOGRAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn in_precision(&self) -> IN_PRECISION_R {
        IN_PRECISION_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn out_precision(&self) -> OUT_PRECISION_R {
        OUT_PRECISION_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn batch_number(&self) -> BATCH_NUMBER_R {
        BATCH_NUMBER_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_feature_mode_cfg](index.html) module"]
pub struct D_FEATURE_MODE_CFG_SPEC;
impl crate::RegisterSpec for D_FEATURE_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_feature_mode_cfg::R](R) reader structure"]
impl crate::Readable for D_FEATURE_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_FEATURE_MODE_CFG to value 0x14"]
impl crate::Resettable for D_FEATURE_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
