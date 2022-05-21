#[doc = "Register `D_WEIGHT_FORMAT` reader"]
pub struct R(crate::R<D_WEIGHT_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WEIGHT_FORMAT_A {
    #[doc = "1: `1`"]
    COMPRESSED = 1,
    #[doc = "0: `0`"]
    UNCOMPRESSED = 0,
}
impl From<WEIGHT_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: WEIGHT_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WEIGHT_FORMAT` reader - "]
pub struct WEIGHT_FORMAT_R(crate::FieldReader<bool, WEIGHT_FORMAT_A>);
impl WEIGHT_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WEIGHT_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEIGHT_FORMAT_A {
        match self.bits {
            true => WEIGHT_FORMAT_A::COMPRESSED,
            false => WEIGHT_FORMAT_A::UNCOMPRESSED,
        }
    }
    #[doc = "Checks if the value of the field is `COMPRESSED`"]
    #[inline(always)]
    pub fn is_compressed(&self) -> bool {
        **self == WEIGHT_FORMAT_A::COMPRESSED
    }
    #[doc = "Checks if the value of the field is `UNCOMPRESSED`"]
    #[inline(always)]
    pub fn is_uncompressed(&self) -> bool {
        **self == WEIGHT_FORMAT_A::UNCOMPRESSED
    }
}
impl core::ops::Deref for WEIGHT_FORMAT_R {
    type Target = crate::FieldReader<bool, WEIGHT_FORMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn weight_format(&self) -> WEIGHT_FORMAT_R {
        WEIGHT_FORMAT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Whether weight is compressed or not\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_format](index.html) module"]
pub struct D_WEIGHT_FORMAT_SPEC;
impl crate::RegisterSpec for D_WEIGHT_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_format::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_WEIGHT_FORMAT to value 0"]
impl crate::Resettable for D_WEIGHT_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
