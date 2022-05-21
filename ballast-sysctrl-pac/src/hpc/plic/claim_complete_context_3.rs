#[doc = "Register `claim_complete_context_3` reader"]
pub struct R(crate::R<CLAIM_COMPLETE_CONTEXT_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIM_COMPLETE_CONTEXT_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIM_COMPLETE_CONTEXT_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIM_COMPLETE_CONTEXT_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `claim_complete_context_3` writer"]
pub struct W(crate::W<CLAIM_COMPLETE_CONTEXT_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIM_COMPLETE_CONTEXT_3_SPEC>;
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
impl From<crate::W<CLAIM_COMPLETE_CONTEXT_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIM_COMPLETE_CONTEXT_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `claim_complete_context_0` reader - "]
pub struct CLAIM_COMPLETE_CONTEXT_0_R(crate::FieldReader<u32, u32>);
impl CLAIM_COMPLETE_CONTEXT_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLAIM_COMPLETE_CONTEXT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLAIM_COMPLETE_CONTEXT_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `claim_complete_context_0` writer - "]
pub struct CLAIM_COMPLETE_CONTEXT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIM_COMPLETE_CONTEXT_0_W<'a> {
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
    pub fn claim_complete_context_0(&self) -> CLAIM_COMPLETE_CONTEXT_0_R {
        CLAIM_COMPLETE_CONTEXT_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claim_complete_context_0(&mut self) -> CLAIM_COMPLETE_CONTEXT_0_W {
        CLAIM_COMPLETE_CONTEXT_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claim_complete_context_3](index.html) module"]
pub struct CLAIM_COMPLETE_CONTEXT_3_SPEC;
impl crate::RegisterSpec for CLAIM_COMPLETE_CONTEXT_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claim_complete_context_3::R](R) reader structure"]
impl crate::Readable for CLAIM_COMPLETE_CONTEXT_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claim_complete_context_3::W](W) writer structure"]
impl crate::Writable for CLAIM_COMPLETE_CONTEXT_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets claim_complete_context_3 to value 0"]
impl crate::Resettable for CLAIM_COMPLETE_CONTEXT_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
