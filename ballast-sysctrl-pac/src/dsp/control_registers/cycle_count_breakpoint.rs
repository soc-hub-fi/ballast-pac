#[doc = "Register `cycle_count_breakpoint` reader"]
pub struct R(crate::R<CYCLE_COUNT_BREAKPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE_COUNT_BREAKPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE_COUNT_BREAKPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE_COUNT_BREAKPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cycle_count_breakpoint` writer"]
pub struct W(crate::W<CYCLE_COUNT_BREAKPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCLE_COUNT_BREAKPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CYCLE_COUNT_BREAKPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCLE_COUNT_BREAKPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cycle_count_breakpoint` reader - "]
pub struct CYCLE_COUNT_BREAKPOINT_R(crate::FieldReader<u32, u32>);
impl CYCLE_COUNT_BREAKPOINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CYCLE_COUNT_BREAKPOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLE_COUNT_BREAKPOINT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cycle_count_breakpoint` writer - "]
pub struct CYCLE_COUNT_BREAKPOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLE_COUNT_BREAKPOINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cycle_count_breakpoint(&self) -> CYCLE_COUNT_BREAKPOINT_R {
        CYCLE_COUNT_BREAKPOINT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cycle_count_breakpoint(&mut self) -> CYCLE_COUNT_BREAKPOINT_W {
        CYCLE_COUNT_BREAKPOINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle_count_breakpoint](index.html) module"]
pub struct CYCLE_COUNT_BREAKPOINT_SPEC;
impl crate::RegisterSpec for CYCLE_COUNT_BREAKPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle_count_breakpoint::R](R) reader structure"]
impl crate::Readable for CYCLE_COUNT_BREAKPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cycle_count_breakpoint::W](W) writer structure"]
impl crate::Writable for CYCLE_COUNT_BREAKPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cycle_count_breakpoint to value 0"]
impl crate::Resettable for CYCLE_COUNT_BREAKPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
