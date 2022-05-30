#[doc = "Register `D_NAN_FLUSH_TO_ZERO` reader"]
pub struct R(crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_NAN_FLUSH_TO_ZERO_SPEC>) -> Self {
        R(reader)
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
pub struct NAN_TO_ZERO_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nan_to_zero(&self) -> NAN_TO_ZERO_R {
        NAN_TO_ZERO_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Option to flush input NaN to zero\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_nan_flush_to_zero](index.html) module"]
pub struct D_NAN_FLUSH_TO_ZERO_SPEC;
impl crate::RegisterSpec for D_NAN_FLUSH_TO_ZERO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_nan_flush_to_zero::R](R) reader structure"]
impl crate::Readable for D_NAN_FLUSH_TO_ZERO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_NAN_FLUSH_TO_ZERO to value 0"]
impl crate::Resettable for D_NAN_FLUSH_TO_ZERO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
