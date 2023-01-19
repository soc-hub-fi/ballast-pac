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
pub type PAUSE_FRAME_SEND_EN_R = crate::BitReader<bool>;
#[doc = "Field `pause_frame_send_en` writer - pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
pub type PAUSE_FRAME_SEND_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAUSE_FRAME_SEND_EN_SPEC, bool, O>;
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
    #[must_use]
    pub fn pause_frame_send_en(&mut self) -> PAUSE_FRAME_SEND_EN_W<0> {
        PAUSE_FRAME_SEND_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pause_frame_send_en to value 0"]
impl crate::Resettable for PAUSE_FRAME_SEND_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
