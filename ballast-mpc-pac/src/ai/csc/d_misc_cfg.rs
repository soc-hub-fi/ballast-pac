#[doc = "Register `D_MISC_CFG` reader"]
pub struct R(crate::R<D_MISC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_MISC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_MISC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_MISC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONV_MODE_A {
    #[doc = "0: `0`"]
    DIRECT = 0,
    #[doc = "1: `1`"]
    WINOGRAD = 1,
}
impl From<CONV_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CONV_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONV_MODE` reader - "]
pub struct CONV_MODE_R(crate::FieldReader<bool, CONV_MODE_A>);
impl CONV_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONV_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONV_MODE_A {
        match self.bits {
            false => CONV_MODE_A::DIRECT,
            true => CONV_MODE_A::WINOGRAD,
        }
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        **self == CONV_MODE_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `WINOGRAD`"]
    #[inline(always)]
    pub fn is_winograd(&self) -> bool {
        **self == CONV_MODE_A::WINOGRAD
    }
}
impl core::ops::Deref for CONV_MODE_R {
    type Target = crate::FieldReader<bool, CONV_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
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
#[doc = "\n\nValue on reset: 0"]
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
pub enum DATA_REUSE_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<DATA_REUSE_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_REUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_REUSE` reader - "]
pub struct DATA_REUSE_R(crate::FieldReader<bool, DATA_REUSE_A>);
impl DATA_REUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_REUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_REUSE_A {
        match self.bits {
            true => DATA_REUSE_A::ENABLE,
            false => DATA_REUSE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DATA_REUSE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DATA_REUSE_A::DISABLE
    }
}
impl core::ops::Deref for DATA_REUSE_R {
    type Target = crate::FieldReader<bool, DATA_REUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WEIGHT_REUSE_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<WEIGHT_REUSE_A> for bool {
    #[inline(always)]
    fn from(variant: WEIGHT_REUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WEIGHT_REUSE` reader - "]
pub struct WEIGHT_REUSE_R(crate::FieldReader<bool, WEIGHT_REUSE_A>);
impl WEIGHT_REUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WEIGHT_REUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEIGHT_REUSE_A {
        match self.bits {
            true => WEIGHT_REUSE_A::ENABLE,
            false => WEIGHT_REUSE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WEIGHT_REUSE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WEIGHT_REUSE_A::DISABLE
    }
}
impl core::ops::Deref for WEIGHT_REUSE_R {
    type Target = crate::FieldReader<bool, WEIGHT_REUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKIP_DATA_RLS_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<SKIP_DATA_RLS_A> for bool {
    #[inline(always)]
    fn from(variant: SKIP_DATA_RLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKIP_DATA_RLS` reader - "]
pub struct SKIP_DATA_RLS_R(crate::FieldReader<bool, SKIP_DATA_RLS_A>);
impl SKIP_DATA_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKIP_DATA_RLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIP_DATA_RLS_A {
        match self.bits {
            true => SKIP_DATA_RLS_A::ENABLE,
            false => SKIP_DATA_RLS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SKIP_DATA_RLS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SKIP_DATA_RLS_A::DISABLE
    }
}
impl core::ops::Deref for SKIP_DATA_RLS_R {
    type Target = crate::FieldReader<bool, SKIP_DATA_RLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKIP_WEIGHT_RLS_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<SKIP_WEIGHT_RLS_A> for bool {
    #[inline(always)]
    fn from(variant: SKIP_WEIGHT_RLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKIP_WEIGHT_RLS` reader - "]
pub struct SKIP_WEIGHT_RLS_R(crate::FieldReader<bool, SKIP_WEIGHT_RLS_A>);
impl SKIP_WEIGHT_RLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKIP_WEIGHT_RLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIP_WEIGHT_RLS_A {
        match self.bits {
            true => SKIP_WEIGHT_RLS_A::ENABLE,
            false => SKIP_WEIGHT_RLS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SKIP_WEIGHT_RLS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SKIP_WEIGHT_RLS_A::DISABLE
    }
}
impl core::ops::Deref for SKIP_WEIGHT_RLS_R {
    type Target = crate::FieldReader<bool, SKIP_WEIGHT_RLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn conv_mode(&self) -> CONV_MODE_R {
        CONV_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn in_precision(&self) -> IN_PRECISION_R {
        IN_PRECISION_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn data_reuse(&self) -> DATA_REUSE_R {
        DATA_REUSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn weight_reuse(&self) -> WEIGHT_REUSE_R {
        WEIGHT_REUSE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn skip_data_rls(&self) -> SKIP_DATA_RLS_R {
        SKIP_DATA_RLS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn skip_weight_rls(&self) -> SKIP_WEIGHT_RLS_R {
        SKIP_WEIGHT_RLS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Configuration of operation: convolution mode, precision, weight reuse, data reuse.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_misc_cfg](index.html) module"]
pub struct D_MISC_CFG_SPEC;
impl crate::RegisterSpec for D_MISC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_misc_cfg::R](R) reader structure"]
impl crate::Readable for D_MISC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_MISC_CFG to value 0"]
impl crate::Resettable for D_MISC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
