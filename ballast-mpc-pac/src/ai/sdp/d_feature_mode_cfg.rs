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
    #[doc = "1: `1`"]
    ON = 1,
    #[doc = "0: `0`"]
    OFF = 0,
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
            true => FLYING_MODE_A::ON,
            false => FLYING_MODE_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == FLYING_MODE_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == FLYING_MODE_A::OFF
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
pub enum OUTPUT_DST_A {
    #[doc = "0: `0`"]
    MEM = 0,
    #[doc = "1: `1`"]
    PDP = 1,
}
impl From<OUTPUT_DST_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_DST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUT_DST` reader - "]
pub struct OUTPUT_DST_R(crate::FieldReader<bool, OUTPUT_DST_A>);
impl OUTPUT_DST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_DST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUT_DST_A {
        match self.bits {
            false => OUTPUT_DST_A::MEM,
            true => OUTPUT_DST_A::PDP,
        }
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        **self == OUTPUT_DST_A::MEM
    }
    #[doc = "Checks if the value of the field is `PDP`"]
    #[inline(always)]
    pub fn is_pdp(&self) -> bool {
        **self == OUTPUT_DST_A::PDP
    }
}
impl core::ops::Deref for OUTPUT_DST_R {
    type Target = crate::FieldReader<bool, OUTPUT_DST_A>;
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAN_TO_ZERO_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<NAN_TO_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: NAN_TO_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAN_TO_ZERO` reader - "]
pub struct NAN_TO_ZERO_R(crate::FieldReader<bool, NAN_TO_ZERO_A>);
impl NAN_TO_ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAN_TO_ZERO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAN_TO_ZERO_A {
        match self.bits {
            true => NAN_TO_ZERO_A::ENABLE,
            false => NAN_TO_ZERO_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == NAN_TO_ZERO_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == NAN_TO_ZERO_A::DISABLE
    }
}
impl core::ops::Deref for NAN_TO_ZERO_R {
    type Target = crate::FieldReader<bool, NAN_TO_ZERO_A>;
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
    pub fn output_dst(&self) -> OUTPUT_DST_R {
        OUTPUT_DST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn winograd(&self) -> WINOGRAD_R {
        WINOGRAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nan_to_zero(&self) -> NAN_TO_ZERO_R {
        NAN_TO_ZERO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn batch_number(&self) -> BATCH_NUMBER_R {
        BATCH_NUMBER_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Operation configuration: flying mode, output destination, Direct or Winograd mode, flush NaN to zero, batch number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_feature_mode_cfg](index.html) module"]
pub struct D_FEATURE_MODE_CFG_SPEC;
impl crate::RegisterSpec for D_FEATURE_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_feature_mode_cfg::R](R) reader structure"]
impl crate::Readable for D_FEATURE_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_FEATURE_MODE_CFG to value 0"]
impl crate::Resettable for D_FEATURE_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
