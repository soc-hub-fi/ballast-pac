#[doc = "Register `FullDuplex` reader"]
pub struct R(crate::R<FULL_DUPLEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FULL_DUPLEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FULL_DUPLEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FULL_DUPLEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FullDuplex` writer"]
pub struct W(crate::W<FULL_DUPLEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FULL_DUPLEX_SPEC>;
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
impl From<crate::W<FULL_DUPLEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FULL_DUPLEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FullDuplex` reader - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
pub type FULL_DUPLEX_R = crate::BitReader<bool>;
#[doc = "Field `FullDuplex` writer - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
pub type FULL_DUPLEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FULL_DUPLEX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
    #[inline(always)]
    pub fn full_duplex(&self) -> FULL_DUPLEX_R {
        FULL_DUPLEX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
    #[inline(always)]
    #[must_use]
    pub fn full_duplex(&mut self) -> FULL_DUPLEX_W<0> {
        FULL_DUPLEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [full_duplex](index.html) module"]
pub struct FULL_DUPLEX_SPEC;
impl crate::RegisterSpec for FULL_DUPLEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [full_duplex::R](R) reader structure"]
impl crate::Readable for FULL_DUPLEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [full_duplex::W](W) writer structure"]
impl crate::Writable for FULL_DUPLEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FullDuplex to value 0x01"]
impl crate::Resettable for FULL_DUPLEX_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
