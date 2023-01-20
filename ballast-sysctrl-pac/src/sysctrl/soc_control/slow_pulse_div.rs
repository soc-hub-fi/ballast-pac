#[doc = "Register `SLOW_PULSE_DIV` reader"]
pub struct R(crate::R<SLOW_PULSE_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOW_PULSE_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOW_PULSE_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOW_PULSE_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOW_PULSE_DIV` writer"]
pub struct W(crate::W<SLOW_PULSE_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOW_PULSE_DIV_SPEC>;
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
impl From<crate::W<SLOW_PULSE_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOW_PULSE_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_PULSE_DIV` reader - "]
pub type SLOW_PULSE_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLOW_PULSE_DIV` writer - "]
pub type SLOW_PULSE_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLOW_PULSE_DIV_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slow_pulse_div(&self) -> SLOW_PULSE_DIV_R {
        SLOW_PULSE_DIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn slow_pulse_div(&mut self) -> SLOW_PULSE_DIV_W<0> {
        SLOW_PULSE_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_pulse_div](index.html) module"]
pub struct SLOW_PULSE_DIV_SPEC;
impl crate::RegisterSpec for SLOW_PULSE_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slow_pulse_div::R](R) reader structure"]
impl crate::Readable for SLOW_PULSE_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slow_pulse_div::W](W) writer structure"]
impl crate::Writable for SLOW_PULSE_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOW_PULSE_DIV to value 0"]
impl crate::Resettable for SLOW_PULSE_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
