#[doc = "Register `timer1_timer` reader"]
pub struct R(crate::R<TIMER1_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer1_timer` writer"]
pub struct W(crate::W<TIMER1_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_TIMER_SPEC>;
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
impl From<crate::W<TIMER1_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer1_timer` reader - "]
pub struct TIMER1_TIMER_R(crate::FieldReader<u32>);
impl TIMER1_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIMER1_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TIMER_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer1_timer` writer - "]
pub struct TIMER1_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_TIMER_W<'a> {
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
    pub fn timer1_timer(&self) -> TIMER1_TIMER_R {
        TIMER1_TIMER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer1_timer(&mut self) -> TIMER1_TIMER_W {
        TIMER1_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_timer](index.html) module"]
pub struct TIMER1_TIMER_SPEC;
impl crate::RegisterSpec for TIMER1_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_timer::R](R) reader structure"]
impl crate::Readable for TIMER1_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_timer::W](W) writer structure"]
impl crate::Writable for TIMER1_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets timer1_timer to value 0"]
impl crate::Resettable for TIMER1_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
