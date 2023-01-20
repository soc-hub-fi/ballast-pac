#[doc = "Register `MaxRetry` reader"]
pub struct R(crate::R<MAX_RETRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_RETRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_RETRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_RETRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MaxRetry` writer"]
pub struct W(crate::W<MAX_RETRY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAX_RETRY_SPEC>;
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
impl From<crate::W<MAX_RETRY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAX_RETRY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MaxRetry` reader - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
pub type MAX_RETRY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MaxRetry` writer - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
pub type MAX_RETRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAX_RETRY_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
    #[inline(always)]
    pub fn max_retry(&self) -> MAX_RETRY_R {
        MAX_RETRY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
    #[inline(always)]
    #[must_use]
    pub fn max_retry(&mut self) -> MAX_RETRY_W<0> {
        MAX_RETRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_retry](index.html) module"]
pub struct MAX_RETRY_SPEC;
impl crate::RegisterSpec for MAX_RETRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_retry::R](R) reader structure"]
impl crate::Readable for MAX_RETRY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [max_retry::W](W) writer structure"]
impl crate::Writable for MAX_RETRY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MaxRetry to value 0x02"]
impl crate::Resettable for MAX_RETRY_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
