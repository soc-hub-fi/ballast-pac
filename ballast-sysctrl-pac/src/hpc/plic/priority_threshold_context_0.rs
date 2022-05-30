#[doc = "Register `priority_threshold_context_0` reader"]
pub struct R(crate::R<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `priority_threshold_context_0` writer"]
pub struct W(crate::W<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>;
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
impl From<crate::W<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIORITY_THRESHOLD_CONTEXT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `priority_threshold_context_0` reader - "]
pub struct PRIORITY_THRESHOLD_CONTEXT_0_R(crate::FieldReader<u32>);
impl PRIORITY_THRESHOLD_CONTEXT_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRIORITY_THRESHOLD_CONTEXT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY_THRESHOLD_CONTEXT_0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `priority_threshold_context_0` writer - "]
pub struct PRIORITY_THRESHOLD_CONTEXT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_THRESHOLD_CONTEXT_0_W<'a> {
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
    pub fn priority_threshold_context_0(&self) -> PRIORITY_THRESHOLD_CONTEXT_0_R {
        PRIORITY_THRESHOLD_CONTEXT_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn priority_threshold_context_0(&mut self) -> PRIORITY_THRESHOLD_CONTEXT_0_W {
        PRIORITY_THRESHOLD_CONTEXT_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priority_threshold_context_0](index.html) module"]
pub struct PRIORITY_THRESHOLD_CONTEXT_0_SPEC;
impl crate::RegisterSpec for PRIORITY_THRESHOLD_CONTEXT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priority_threshold_context_0::R](R) reader structure"]
impl crate::Readable for PRIORITY_THRESHOLD_CONTEXT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priority_threshold_context_0::W](W) writer structure"]
impl crate::Writable for PRIORITY_THRESHOLD_CONTEXT_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets priority_threshold_context_0 to value 0"]
impl crate::Resettable for PRIORITY_THRESHOLD_CONTEXT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
