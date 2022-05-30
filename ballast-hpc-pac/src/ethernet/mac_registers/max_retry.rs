#[doc = "Register `MaxRetry` reader"]
pub struct R(crate::R<MAXRETRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXRETRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXRETRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXRETRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MaxRetry` writer"]
pub struct W(crate::W<MAXRETRY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXRETRY_SPEC>;
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
impl From<crate::W<MAXRETRY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXRETRY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MaxRetry` reader - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
pub struct MAXRETRY_R(crate::FieldReader<u8>);
impl MAXRETRY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXRETRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXRETRY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MaxRetry` writer - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
pub struct MAXRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXRETRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
    #[inline(always)]
    pub fn max_retry(&self) -> MAXRETRY_R {
        MAXRETRY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
    #[inline(always)]
    pub fn max_retry(&mut self) -> MAXRETRY_W {
        MAXRETRY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_retry](index.html) module"]
pub struct MAXRETRY_SPEC;
impl crate::RegisterSpec for MAXRETRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_retry::R](R) reader structure"]
impl crate::Readable for MAXRETRY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [max_retry::W](W) writer structure"]
impl crate::Writable for MAXRETRY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MaxRetry to value 0x02"]
impl crate::Resettable for MAXRETRY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
