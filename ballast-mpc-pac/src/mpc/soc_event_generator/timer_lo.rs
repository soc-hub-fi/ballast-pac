#[doc = "Register `TIMER_LO` reader"]
pub struct R(crate::R<TIMER_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_LO` writer"]
pub struct W(crate::W<TIMER_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_LO_SPEC>;
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
impl From<crate::W<TIMER_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_LO_EVENT` reader - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
pub struct TIMER_LO_EVENT_R(crate::FieldReader<u8>);
impl TIMER_LO_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_LO_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_LO_EVENT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_LO_EVENT` writer - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
pub struct TIMER_LO_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_LO_EVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
    #[inline(always)]
    pub fn timer_lo_event(&self) -> TIMER_LO_EVENT_R {
        TIMER_LO_EVENT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
    #[inline(always)]
    pub fn timer_lo_event(&mut self) -> TIMER_LO_EVENT_W {
        TIMER_LO_EVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_lo](index.html) module"]
pub struct TIMER_LO_SPEC;
impl crate::RegisterSpec for TIMER_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_lo::R](R) reader structure"]
impl crate::Readable for TIMER_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_lo::W](W) writer structure"]
impl crate::Writable for TIMER_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_LO to value 0"]
impl crate::Resettable for TIMER_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
