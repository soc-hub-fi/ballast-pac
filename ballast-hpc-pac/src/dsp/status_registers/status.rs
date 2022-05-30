#[doc = "Register `status` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `single_stepping_breakpoint` reader - "]
pub struct SINGLE_STEPPING_BREAKPOINT_R(crate::FieldReader<bool>);
impl SINGLE_STEPPING_BREAKPOINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_STEPPING_BREAKPOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_STEPPING_BREAKPOINT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `breakpoint_1` reader - "]
pub struct BREAKPOINT_1_R(crate::FieldReader<bool>);
impl BREAKPOINT_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAKPOINT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAKPOINT_1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `manual_breakpoint` reader - "]
pub struct MANUAL_BREAKPOINT_R(crate::FieldReader<bool>);
impl MANUAL_BREAKPOINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MANUAL_BREAKPOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANUAL_BREAKPOINT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `breakpoint_2` reader - "]
pub struct BREAKPOINT_2_R(crate::FieldReader<bool>);
impl BREAKPOINT_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAKPOINT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAKPOINT_2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn single_stepping_breakpoint(&self) -> SINGLE_STEPPING_BREAKPOINT_R {
        SINGLE_STEPPING_BREAKPOINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn breakpoint_1(&self) -> BREAKPOINT_1_R {
        BREAKPOINT_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn manual_breakpoint(&self) -> MANUAL_BREAKPOINT_R {
        MANUAL_BREAKPOINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn breakpoint_2(&self) -> BREAKPOINT_2_R {
        BREAKPOINT_2_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
