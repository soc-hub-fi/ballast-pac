#[doc = "Register `pause_quanta_set` reader"]
pub struct R(crate::R<PAUSE_QUANTA_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_QUANTA_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_QUANTA_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_QUANTA_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pause_quanta_set` writer"]
pub struct W(crate::W<PAUSE_QUANTA_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAUSE_QUANTA_SET_SPEC>;
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
impl From<crate::W<PAUSE_QUANTA_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAUSE_QUANTA_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pause_quanta_set` reader - pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
pub type PAUSE_QUANTA_SET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pause_quanta_set` writer - pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
pub type PAUSE_QUANTA_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAUSE_QUANTA_SET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
    #[inline(always)]
    pub fn pause_quanta_set(&self) -> PAUSE_QUANTA_SET_R {
        PAUSE_QUANTA_SET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
    #[inline(always)]
    #[must_use]
    pub fn pause_quanta_set(&mut self) -> PAUSE_QUANTA_SET_W<0> {
        PAUSE_QUANTA_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pause_quanta_set register is used to setting quanta value in send PAUSE frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause_quanta_set](index.html) module"]
pub struct PAUSE_QUANTA_SET_SPEC;
impl crate::RegisterSpec for PAUSE_QUANTA_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause_quanta_set::R](R) reader structure"]
impl crate::Readable for PAUSE_QUANTA_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pause_quanta_set::W](W) writer structure"]
impl crate::Writable for PAUSE_QUANTA_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pause_quanta_set to value 0"]
impl crate::Resettable for PAUSE_QUANTA_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
