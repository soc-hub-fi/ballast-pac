#[doc = "Register `S_STATUS` reader"]
pub struct R(crate::R<S_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATUS_0_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<STATUS_0_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATUS_0` reader - "]
pub struct STATUS_0_R(crate::FieldReader<u8>);
impl STATUS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_0_A> {
        match self.bits {
            1 => Some(STATUS_0_A::ENABLE),
            0 => Some(STATUS_0_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == STATUS_0_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == STATUS_0_A::DISABLE
    }
}
impl core::ops::Deref for STATUS_0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATUS_1_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<STATUS_1_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATUS_1` reader - "]
pub struct STATUS_1_R(crate::FieldReader<u8>);
impl STATUS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_1_A> {
        match self.bits {
            1 => Some(STATUS_1_A::ENABLE),
            0 => Some(STATUS_1_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == STATUS_1_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == STATUS_1_A::DISABLE
    }
}
impl core::ops::Deref for STATUS_1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status_0(&self) -> STATUS_0_R {
        STATUS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn status_1(&self) -> STATUS_1_R {
        STATUS_1_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "Idle status of two register groups\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_status](index.html) module"]
pub struct S_STATUS_SPEC;
impl crate::RegisterSpec for S_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_status::R](R) reader structure"]
impl crate::Readable for S_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_STATUS to value 0"]
impl crate::Resettable for S_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
