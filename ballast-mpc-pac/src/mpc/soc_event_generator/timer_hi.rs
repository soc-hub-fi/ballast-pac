#[doc = "Register `TIMER_HI` reader"]
pub struct R(crate::R<TIMER_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_HI` writer"]
pub struct W(crate::W<TIMER_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_HI_SPEC>;
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
impl From<crate::W<TIMER_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_HI_EVENT` reader - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
pub type TIMER_HI_EVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_HI_EVENT` writer - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
pub type TIMER_HI_EVENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER_HI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
    #[inline(always)]
    pub fn timer_hi_event(&self) -> TIMER_HI_EVENT_R {
        TIMER_HI_EVENT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn timer_hi_event(&mut self) -> TIMER_HI_EVENT_W<0> {
        TIMER_HI_EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Timer HI of APB Timer with event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_hi](index.html) module"]
pub struct TIMER_HI_SPEC;
impl crate::RegisterSpec for TIMER_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_hi::R](R) reader structure"]
impl crate::Readable for TIMER_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_hi::W](W) writer structure"]
impl crate::Writable for TIMER_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_HI to value 0"]
impl crate::Resettable for TIMER_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
