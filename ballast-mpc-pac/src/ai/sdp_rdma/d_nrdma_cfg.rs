#[doc = "Register `D_NRDMA_CFG` reader"]
pub struct R(crate::R<D_NRDMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_NRDMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_NRDMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_NRDMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_DISABLE_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<NRDMA_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_DISABLE` reader - "]
pub struct NRDMA_DISABLE_R(crate::FieldReader<bool>);
impl NRDMA_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_DISABLE_A {
        match self.bits {
            true => NRDMA_DISABLE_A::YES,
            false => NRDMA_DISABLE_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == NRDMA_DISABLE_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == NRDMA_DISABLE_A::NO
    }
}
impl core::ops::Deref for NRDMA_DISABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NRDMA_DATA_USE_A {
    #[doc = "0: `0`"]
    MUL = 0,
    #[doc = "2: `10`"]
    BOTH = 2,
    #[doc = "1: `1`"]
    ALU = 1,
}
impl From<NRDMA_DATA_USE_A> for u8 {
    #[inline(always)]
    fn from(variant: NRDMA_DATA_USE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NRDMA_DATA_USE` reader - "]
pub struct NRDMA_DATA_USE_R(crate::FieldReader<u8>);
impl NRDMA_DATA_USE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NRDMA_DATA_USE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NRDMA_DATA_USE_A> {
        match self.bits {
            0 => Some(NRDMA_DATA_USE_A::MUL),
            2 => Some(NRDMA_DATA_USE_A::BOTH),
            1 => Some(NRDMA_DATA_USE_A::ALU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        **self == NRDMA_DATA_USE_A::MUL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == NRDMA_DATA_USE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `ALU`"]
    #[inline(always)]
    pub fn is_alu(&self) -> bool {
        **self == NRDMA_DATA_USE_A::ALU
    }
}
impl core::ops::Deref for NRDMA_DATA_USE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_DATA_SIZE_A {
    #[doc = "1: `1`"]
    TWO_BYTE = 1,
    #[doc = "0: `0`"]
    ONE_BYTE = 0,
}
impl From<NRDMA_DATA_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_DATA_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_DATA_SIZE` reader - "]
pub struct NRDMA_DATA_SIZE_R(crate::FieldReader<bool>);
impl NRDMA_DATA_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_DATA_SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_DATA_SIZE_A {
        match self.bits {
            true => NRDMA_DATA_SIZE_A::TWO_BYTE,
            false => NRDMA_DATA_SIZE_A::ONE_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_BYTE`"]
    #[inline(always)]
    pub fn is_two_byte(&self) -> bool {
        **self == NRDMA_DATA_SIZE_A::TWO_BYTE
    }
    #[doc = "Checks if the value of the field is `ONE_BYTE`"]
    #[inline(always)]
    pub fn is_one_byte(&self) -> bool {
        **self == NRDMA_DATA_SIZE_A::ONE_BYTE
    }
}
impl core::ops::Deref for NRDMA_DATA_SIZE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_DATA_MODE_A {
    #[doc = "0: `0`"]
    PER_KERNEL = 0,
    #[doc = "1: `1`"]
    PER_ELEMENT = 1,
}
impl From<NRDMA_DATA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_DATA_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_DATA_MODE` reader - "]
pub struct NRDMA_DATA_MODE_R(crate::FieldReader<bool>);
impl NRDMA_DATA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_DATA_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_DATA_MODE_A {
        match self.bits {
            false => NRDMA_DATA_MODE_A::PER_KERNEL,
            true => NRDMA_DATA_MODE_A::PER_ELEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `PER_KERNEL`"]
    #[inline(always)]
    pub fn is_per_kernel(&self) -> bool {
        **self == NRDMA_DATA_MODE_A::PER_KERNEL
    }
    #[doc = "Checks if the value of the field is `PER_ELEMENT`"]
    #[inline(always)]
    pub fn is_per_element(&self) -> bool {
        **self == NRDMA_DATA_MODE_A::PER_ELEMENT
    }
}
impl core::ops::Deref for NRDMA_DATA_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRDMA_RAM_TYPE_A {
    #[doc = "1: `1`"]
    MC = 1,
    #[doc = "0: `0`"]
    CV = 0,
}
impl From<NRDMA_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDMA_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDMA_RAM_TYPE` reader - "]
pub struct NRDMA_RAM_TYPE_R(crate::FieldReader<bool>);
impl NRDMA_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRDMA_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRDMA_RAM_TYPE_A {
        match self.bits {
            true => NRDMA_RAM_TYPE_A::MC,
            false => NRDMA_RAM_TYPE_A::CV,
        }
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        **self == NRDMA_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        **self == NRDMA_RAM_TYPE_A::CV
    }
}
impl core::ops::Deref for NRDMA_RAM_TYPE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nrdma_disable(&self) -> NRDMA_DISABLE_R {
        NRDMA_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn nrdma_data_use(&self) -> NRDMA_DATA_USE_R {
        NRDMA_DATA_USE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn nrdma_data_size(&self) -> NRDMA_DATA_SIZE_R {
        NRDMA_DATA_SIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nrdma_data_mode(&self) -> NRDMA_DATA_MODE_R {
        NRDMA_DATA_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nrdma_ram_type(&self) -> NRDMA_RAM_TYPE_R {
        NRDMA_RAM_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_nrdma_cfg](index.html) module"]
pub struct D_NRDMA_CFG_SPEC;
impl crate::RegisterSpec for D_NRDMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_nrdma_cfg::R](R) reader structure"]
impl crate::Readable for D_NRDMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_NRDMA_CFG to value 0x01"]
impl crate::Resettable for D_NRDMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
