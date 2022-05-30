#[doc = "Register `D_DATA_FORMAT` reader"]
pub struct R(crate::R<D_DATA_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATA_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATA_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATA_FORMAT_SPEC>) -> Self {
        R(reader)
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
pub struct OUT_PRECISION_R(crate::FieldReader<u8>);
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
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn out_precision(&self) -> OUT_PRECISION_R {
        OUT_PRECISION_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[doc = "Data precision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_data_format](index.html) module"]
pub struct D_DATA_FORMAT_SPEC;
impl crate::RegisterSpec for D_DATA_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_data_format::R](R) reader structure"]
impl crate::Readable for D_DATA_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATA_FORMAT to value 0"]
impl crate::Resettable for D_DATA_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
