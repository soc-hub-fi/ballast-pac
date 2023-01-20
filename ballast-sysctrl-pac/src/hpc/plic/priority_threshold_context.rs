#[doc = "Register `priority_threshold_context[%s]` reader"]
pub struct R(crate::R<PRIORITY_THRESHOLD_CONTEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIORITY_THRESHOLD_CONTEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIORITY_THRESHOLD_CONTEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIORITY_THRESHOLD_CONTEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `priority_threshold_context[%s]` writer"]
pub struct W(crate::W<PRIORITY_THRESHOLD_CONTEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIORITY_THRESHOLD_CONTEXT_SPEC>;
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
impl From<crate::W<PRIORITY_THRESHOLD_CONTEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIORITY_THRESHOLD_CONTEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `priority_threshold_context` reader - "]
pub type PRIORITY_THRESHOLD_CONTEXT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `priority_threshold_context` writer - "]
pub type PRIORITY_THRESHOLD_CONTEXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRIORITY_THRESHOLD_CONTEXT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn priority_threshold_context(&self) -> PRIORITY_THRESHOLD_CONTEXT_R {
        PRIORITY_THRESHOLD_CONTEXT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn priority_threshold_context(&mut self) -> PRIORITY_THRESHOLD_CONTEXT_W<0> {
        PRIORITY_THRESHOLD_CONTEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priority_threshold_context](index.html) module"]
pub struct PRIORITY_THRESHOLD_CONTEXT_SPEC;
impl crate::RegisterSpec for PRIORITY_THRESHOLD_CONTEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priority_threshold_context::R](R) reader structure"]
impl crate::Readable for PRIORITY_THRESHOLD_CONTEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priority_threshold_context::W](W) writer structure"]
impl crate::Writable for PRIORITY_THRESHOLD_CONTEXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets priority_threshold_context[%s]
to value 0"]
impl crate::Resettable for PRIORITY_THRESHOLD_CONTEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
