#[doc = "Register `D_DATAIN_FORMAT` reader"]
pub struct R(crate::R<D_DATAIN_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAIN_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAIN_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAIN_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAIN_FORMAT_A {
    #[doc = "1: `1`"]
    PIXEL = 1,
    #[doc = "0: `0`"]
    FEATURE = 0,
}
impl From<DATAIN_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DATAIN_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAIN_FORMAT` reader - "]
pub struct DATAIN_FORMAT_R(crate::FieldReader<bool>);
impl DATAIN_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAIN_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAIN_FORMAT_A {
        match self.bits {
            true => DATAIN_FORMAT_A::PIXEL,
            false => DATAIN_FORMAT_A::FEATURE,
        }
    }
    #[doc = "Checks if the value of the field is `PIXEL`"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        **self == DATAIN_FORMAT_A::PIXEL
    }
    #[doc = "Checks if the value of the field is `FEATURE`"]
    #[inline(always)]
    pub fn is_feature(&self) -> bool {
        **self == DATAIN_FORMAT_A::FEATURE
    }
}
impl core::ops::Deref for DATAIN_FORMAT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn datain_format(&self) -> DATAIN_FORMAT_R {
        DATAIN_FORMAT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Input data format and pixel format\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_datain_format](index.html) module"]
pub struct D_DATAIN_FORMAT_SPEC;
impl crate::RegisterSpec for D_DATAIN_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_datain_format::R](R) reader structure"]
impl crate::Readable for D_DATAIN_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATAIN_FORMAT to value 0"]
impl crate::Resettable for D_DATAIN_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
