#[doc = "Register `D_ERDMA_CFG` reader"]
pub struct R(crate::R<D_ERDMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ERDMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ERDMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ERDMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERDMA_DISABLE_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<ERDMA_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_DISABLE` reader - "]
pub struct ERDMA_DISABLE_R(crate::FieldReader<bool, ERDMA_DISABLE_A>);
impl ERDMA_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_DISABLE_A {
        match self.bits {
            true => ERDMA_DISABLE_A::YES,
            false => ERDMA_DISABLE_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ERDMA_DISABLE_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ERDMA_DISABLE_A::NO
    }
}
impl core::ops::Deref for ERDMA_DISABLE_R {
    type Target = crate::FieldReader<bool, ERDMA_DISABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERDMA_DATA_USE_A {
    #[doc = "0: `0`"]
    MUL = 0,
    #[doc = "2: `10`"]
    BOTH = 2,
    #[doc = "1: `1`"]
    ALU = 1,
}
impl From<ERDMA_DATA_USE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERDMA_DATA_USE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ERDMA_DATA_USE` reader - "]
pub struct ERDMA_DATA_USE_R(crate::FieldReader<u8, ERDMA_DATA_USE_A>);
impl ERDMA_DATA_USE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERDMA_DATA_USE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERDMA_DATA_USE_A> {
        match self.bits {
            0 => Some(ERDMA_DATA_USE_A::MUL),
            2 => Some(ERDMA_DATA_USE_A::BOTH),
            1 => Some(ERDMA_DATA_USE_A::ALU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        **self == ERDMA_DATA_USE_A::MUL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == ERDMA_DATA_USE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `ALU`"]
    #[inline(always)]
    pub fn is_alu(&self) -> bool {
        **self == ERDMA_DATA_USE_A::ALU
    }
}
impl core::ops::Deref for ERDMA_DATA_USE_R {
    type Target = crate::FieldReader<u8, ERDMA_DATA_USE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERDMA_DATA_SIZE_A {
    #[doc = "1: `1`"]
    TWO_BYTE = 1,
    #[doc = "0: `0`"]
    ONE_BYTE = 0,
}
impl From<ERDMA_DATA_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_DATA_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_DATA_SIZE` reader - "]
pub struct ERDMA_DATA_SIZE_R(crate::FieldReader<bool, ERDMA_DATA_SIZE_A>);
impl ERDMA_DATA_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_DATA_SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_DATA_SIZE_A {
        match self.bits {
            true => ERDMA_DATA_SIZE_A::TWO_BYTE,
            false => ERDMA_DATA_SIZE_A::ONE_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_BYTE`"]
    #[inline(always)]
    pub fn is_two_byte(&self) -> bool {
        **self == ERDMA_DATA_SIZE_A::TWO_BYTE
    }
    #[doc = "Checks if the value of the field is `ONE_BYTE`"]
    #[inline(always)]
    pub fn is_one_byte(&self) -> bool {
        **self == ERDMA_DATA_SIZE_A::ONE_BYTE
    }
}
impl core::ops::Deref for ERDMA_DATA_SIZE_R {
    type Target = crate::FieldReader<bool, ERDMA_DATA_SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERDMA_DATA_MODE_A {
    #[doc = "0: `0`"]
    PER_KERNEL = 0,
    #[doc = "1: `1`"]
    PER_ELEMENT = 1,
}
impl From<ERDMA_DATA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_DATA_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_DATA_MODE` reader - "]
pub struct ERDMA_DATA_MODE_R(crate::FieldReader<bool, ERDMA_DATA_MODE_A>);
impl ERDMA_DATA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_DATA_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_DATA_MODE_A {
        match self.bits {
            false => ERDMA_DATA_MODE_A::PER_KERNEL,
            true => ERDMA_DATA_MODE_A::PER_ELEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `PER_KERNEL`"]
    #[inline(always)]
    pub fn is_per_kernel(&self) -> bool {
        **self == ERDMA_DATA_MODE_A::PER_KERNEL
    }
    #[doc = "Checks if the value of the field is `PER_ELEMENT`"]
    #[inline(always)]
    pub fn is_per_element(&self) -> bool {
        **self == ERDMA_DATA_MODE_A::PER_ELEMENT
    }
}
impl core::ops::Deref for ERDMA_DATA_MODE_R {
    type Target = crate::FieldReader<bool, ERDMA_DATA_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERDMA_RAM_TYPE_A {
    #[doc = "1: `1`"]
    MC = 1,
    #[doc = "0: `0`"]
    CV = 0,
}
impl From<ERDMA_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: ERDMA_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERDMA_RAM_TYPE` reader - "]
pub struct ERDMA_RAM_TYPE_R(crate::FieldReader<bool, ERDMA_RAM_TYPE_A>);
impl ERDMA_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERDMA_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERDMA_RAM_TYPE_A {
        match self.bits {
            true => ERDMA_RAM_TYPE_A::MC,
            false => ERDMA_RAM_TYPE_A::CV,
        }
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        **self == ERDMA_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        **self == ERDMA_RAM_TYPE_A::CV
    }
}
impl core::ops::Deref for ERDMA_RAM_TYPE_R {
    type Target = crate::FieldReader<bool, ERDMA_RAM_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn erdma_disable(&self) -> ERDMA_DISABLE_R {
        ERDMA_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn erdma_data_use(&self) -> ERDMA_DATA_USE_R {
        ERDMA_DATA_USE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn erdma_data_size(&self) -> ERDMA_DATA_SIZE_R {
        ERDMA_DATA_SIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erdma_data_mode(&self) -> ERDMA_DATA_MODE_R {
        ERDMA_DATA_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn erdma_ram_type(&self) -> ERDMA_RAM_TYPE_R {
        ERDMA_RAM_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_erdma_cfg](index.html) module"]
pub struct D_ERDMA_CFG_SPEC;
impl crate::RegisterSpec for D_ERDMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_erdma_cfg::R](R) reader structure"]
impl crate::Readable for D_ERDMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_ERDMA_CFG to value 0x01"]
impl crate::Resettable for D_ERDMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
