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
pub struct CONV_MODE_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
pub struct PROC_PRECISION_R(crate::FieldReader<u8>);
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
    type Target = crate::FieldReader<u8>;
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
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[doc = "Configuration of operation: convolution mode, precision, etc.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_misc_cfg](index.html) module"]
pub struct D_MISC_CFG_SPEC;
impl crate::RegisterSpec for D_MISC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_misc_cfg::R](R) reader structure"]
impl crate::Readable for D_MISC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_MISC_CFG to value 0x0010_0000"]
impl crate::Resettable for D_MISC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
