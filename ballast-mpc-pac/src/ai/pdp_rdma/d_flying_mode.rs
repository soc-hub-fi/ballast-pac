#[doc = "Register `D_FLYING_MODE` reader"]
pub struct R(crate::R<D_FLYING_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_FLYING_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_FLYING_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_FLYING_MODE_SPEC>) -> Self {
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
pub struct FLYING_MODE_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_flying_mode](index.html) module"]
pub struct D_FLYING_MODE_SPEC;
impl crate::RegisterSpec for D_FLYING_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_flying_mode::R](R) reader structure"]
impl crate::Readable for D_FLYING_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_FLYING_MODE to value 0"]
impl crate::Resettable for D_FLYING_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
