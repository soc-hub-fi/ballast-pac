#[doc = "Register `EVENT_CFG` reader"]
pub struct R(crate::R<EVENT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENT_CFG` writer"]
pub struct W(crate::W<EVENT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENT_CFG_SPEC>;
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
impl From<crate::W<EVENT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT_CFG` reader - "]
pub type EVENT_CFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EVENT_CFG` writer - "]
pub type EVENT_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVENT_CFG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn event_cfg(&self) -> EVENT_CFG_R {
        EVENT_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn event_cfg(&mut self) -> EVENT_CFG_W<0> {
        EVENT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMERS events configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_cfg](index.html) module"]
pub struct EVENT_CFG_SPEC;
impl crate::RegisterSpec for EVENT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event_cfg::R](R) reader structure"]
impl crate::Readable for EVENT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [event_cfg::W](W) writer structure"]
impl crate::Writable for EVENT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENT_CFG to value 0"]
impl crate::Resettable for EVENT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
