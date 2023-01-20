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
#[doc = "Field `STATUS_0` reader - "]
pub type STATUS_0_R = crate::FieldReader<u8, STATUS_0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATUS_0_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    RUNNING = 1,
    #[doc = "2: `10`"]
    PENDING = 2,
}
impl From<STATUS_0_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_0_A) -> Self {
        variant as _
    }
}
impl STATUS_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_0_A> {
        match self.bits {
            0 => Some(STATUS_0_A::IDLE),
            1 => Some(STATUS_0_A::RUNNING),
            2 => Some(STATUS_0_A::PENDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATUS_0_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATUS_0_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STATUS_0_A::PENDING
    }
}
#[doc = "Field `STATUS_1` reader - "]
pub type STATUS_1_R = crate::FieldReader<u8, STATUS_1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATUS_1_A {
    #[doc = "1: `1`"]
    RUNNING = 1,
    #[doc = "2: `10`"]
    PENDING = 2,
    #[doc = "0: `0`"]
    IDLE = 0,
}
impl From<STATUS_1_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_1_A) -> Self {
        variant as _
    }
}
impl STATUS_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_1_A> {
        match self.bits {
            1 => Some(STATUS_1_A::RUNNING),
            2 => Some(STATUS_1_A::PENDING),
            0 => Some(STATUS_1_A::IDLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATUS_1_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STATUS_1_A::PENDING
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATUS_1_A::IDLE
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
    const RESET_VALUE: Self::Ux = 0;
}
