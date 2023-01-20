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
pub type CYCLE_COUNT_BREAKPOINT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cycle_count_breakpoint` writer - "]
pub type CYCLE_COUNT_BREAKPOINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CYCLE_COUNT_BREAKPOINT_SPEC, u32, u32, 32, O>;
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
    #[must_use]
    pub fn cycle_count_breakpoint(&mut self) -> CYCLE_COUNT_BREAKPOINT_W<0> {
        CYCLE_COUNT_BREAKPOINT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cycle_count_breakpoint to value 0"]
impl crate::Resettable for CYCLE_COUNT_BREAKPOINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
