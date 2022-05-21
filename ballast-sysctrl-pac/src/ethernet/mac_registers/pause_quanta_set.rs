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
pub struct PAUSE_QUANTA_SET_R(crate::FieldReader<u16, u16>);
impl PAUSE_QUANTA_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAUSE_QUANTA_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_QUANTA_SET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pause_quanta_set` writer - pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
pub struct PAUSE_QUANTA_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_QUANTA_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn pause_quanta_set(&mut self) -> PAUSE_QUANTA_SET_W {
        PAUSE_QUANTA_SET_W { w: self }
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
}
#[doc = "`reset()` method sets pause_quanta_set to value 0"]
impl crate::Resettable for PAUSE_QUANTA_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
