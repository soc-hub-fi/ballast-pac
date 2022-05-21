#[doc = "Register `breakpoint_count` reader"]
pub struct R(crate::R<BREAKPOINT_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BREAKPOINT_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BREAKPOINT_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BREAKPOINT_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `breakpoint_count` reader - "]
pub struct BREAKPOINT_COUNT_R(crate::FieldReader<u32, u32>);
impl BREAKPOINT_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BREAKPOINT_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAKPOINT_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn breakpoint_count(&self) -> BREAKPOINT_COUNT_R {
        BREAKPOINT_COUNT_R::new(self.bits)
    }
}
#[doc = "Breakpoint count (0x2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breakpoint_count](index.html) module"]
pub struct BREAKPOINT_COUNT_SPEC;
impl crate::RegisterSpec for BREAKPOINT_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [breakpoint_count::R](R) reader structure"]
impl crate::Readable for BREAKPOINT_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets breakpoint_count to value 0"]
impl crate::Resettable for BREAKPOINT_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
