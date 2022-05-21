#[doc = "Register `pause_frame_send_en` reader"]
pub struct R(crate::R<PAUSE_FRAME_SEND_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_FRAME_SEND_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_FRAME_SEND_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_FRAME_SEND_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pause_frame_send_en` writer"]
pub struct W(crate::W<PAUSE_FRAME_SEND_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAUSE_FRAME_SEND_EN_SPEC>;
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
impl From<crate::W<PAUSE_FRAME_SEND_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAUSE_FRAME_SEND_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pause_frame_send_en` reader - pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
pub struct PAUSE_FRAME_SEND_EN_R(crate::FieldReader<bool, bool>);
impl PAUSE_FRAME_SEND_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUSE_FRAME_SEND_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_FRAME_SEND_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pause_frame_send_en` writer - pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
pub struct PAUSE_FRAME_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_FRAME_SEND_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
    #[inline(always)]
    pub fn pause_frame_send_en(&self) -> PAUSE_FRAME_SEND_EN_R {
        PAUSE_FRAME_SEND_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
    #[inline(always)]
    pub fn pause_frame_send_en(&mut self) -> PAUSE_FRAME_SEND_EN_W {
        PAUSE_FRAME_SEND_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pause_frame_send_en register is used to enable transmit logic to send PAUSE frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause_frame_send_en](index.html) module"]
pub struct PAUSE_FRAME_SEND_EN_SPEC;
impl crate::RegisterSpec for PAUSE_FRAME_SEND_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause_frame_send_en::R](R) reader structure"]
impl crate::Readable for PAUSE_FRAME_SEND_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pause_frame_send_en::W](W) writer structure"]
impl crate::Writable for PAUSE_FRAME_SEND_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pause_frame_send_en to value 0"]
impl crate::Resettable for PAUSE_FRAME_SEND_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
