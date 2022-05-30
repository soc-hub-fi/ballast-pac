#[doc = "Register `D_MEAN_FORMAT` reader"]
pub struct R(crate::R<D_MEAN_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_MEAN_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_MEAN_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_MEAN_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEAN_FORMAT_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<MEAN_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: MEAN_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEAN_FORMAT` reader - "]
pub struct MEAN_FORMAT_R(crate::FieldReader<bool>);
impl MEAN_FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAN_FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEAN_FORMAT_A {
        match self.bits {
            true => MEAN_FORMAT_A::ENABLE,
            false => MEAN_FORMAT_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MEAN_FORMAT_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MEAN_FORMAT_A::DISABLE
    }
}
impl core::ops::Deref for MEAN_FORMAT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mean_format(&self) -> MEAN_FORMAT_R {
        MEAN_FORMAT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Whether mean registers are used or not\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_mean_format](index.html) module"]
pub struct D_MEAN_FORMAT_SPEC;
impl crate::RegisterSpec for D_MEAN_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_mean_format::R](R) reader structure"]
impl crate::Readable for D_MEAN_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_MEAN_FORMAT to value 0"]
impl crate::Resettable for D_MEAN_FORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
