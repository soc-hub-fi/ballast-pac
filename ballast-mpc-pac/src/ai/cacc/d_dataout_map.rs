#[doc = "Register `D_DATAOUT_MAP` reader"]
pub struct R(crate::R<D_DATAOUT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAOUT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAOUT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAOUT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE_PACKED_A {
    #[doc = "1: `1`"]
    TRUE = 1,
    #[doc = "0: `0`"]
    FALSE = 0,
}
impl From<LINE_PACKED_A> for bool {
    #[inline(always)]
    fn from(variant: LINE_PACKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_PACKED` reader - "]
pub struct LINE_PACKED_R(crate::FieldReader<bool, LINE_PACKED_A>);
impl LINE_PACKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINE_PACKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_PACKED_A {
        match self.bits {
            true => LINE_PACKED_A::TRUE,
            false => LINE_PACKED_A::FALSE,
        }
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        **self == LINE_PACKED_A::TRUE
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        **self == LINE_PACKED_A::FALSE
    }
}
impl core::ops::Deref for LINE_PACKED_R {
    type Target = crate::FieldReader<bool, LINE_PACKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SURF_PACKED_A {
    #[doc = "1: `1`"]
    TRUE = 1,
    #[doc = "0: `0`"]
    FALSE = 0,
}
impl From<SURF_PACKED_A> for bool {
    #[inline(always)]
    fn from(variant: SURF_PACKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SURF_PACKED` reader - "]
pub struct SURF_PACKED_R(crate::FieldReader<bool, SURF_PACKED_A>);
impl SURF_PACKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SURF_PACKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SURF_PACKED_A {
        match self.bits {
            true => SURF_PACKED_A::TRUE,
            false => SURF_PACKED_A::FALSE,
        }
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        **self == SURF_PACKED_A::TRUE
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        **self == SURF_PACKED_A::FALSE
    }
}
impl core::ops::Deref for SURF_PACKED_R {
    type Target = crate::FieldReader<bool, SURF_PACKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn line_packed(&self) -> LINE_PACKED_R {
        LINE_PACKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn surf_packed(&self) -> SURF_PACKED_R {
        SURF_PACKED_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Whether output cube is line packed or surface packed\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dataout_map](index.html) module"]
pub struct D_DATAOUT_MAP_SPEC;
impl crate::RegisterSpec for D_DATAOUT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dataout_map::R](R) reader structure"]
impl crate::Readable for D_DATAOUT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATAOUT_MAP to value 0"]
impl crate::Resettable for D_DATAOUT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
