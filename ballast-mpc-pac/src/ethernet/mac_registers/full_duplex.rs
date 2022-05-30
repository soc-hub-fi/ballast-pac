#[doc = "Register `FullDuplex` reader"]
pub struct R(crate::R<FULLDUPLEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FULLDUPLEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FULLDUPLEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FULLDUPLEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FullDuplex` writer"]
pub struct W(crate::W<FULLDUPLEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FULLDUPLEX_SPEC>;
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
impl From<crate::W<FULLDUPLEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FULLDUPLEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FullDuplex` reader - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
pub struct FULLDUPLEX_R(crate::FieldReader<bool>);
impl FULLDUPLEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULLDUPLEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULLDUPLEX_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FullDuplex` writer - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
pub struct FULLDUPLEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUPLEX_W<'a> {
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
    #[doc = "Bit 0 - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
    #[inline(always)]
    pub fn full_duplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
    #[inline(always)]
    pub fn full_duplex(&mut self) -> FULLDUPLEX_W {
        FULLDUPLEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [full_duplex](index.html) module"]
pub struct FULLDUPLEX_SPEC;
impl crate::RegisterSpec for FULLDUPLEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [full_duplex::R](R) reader structure"]
impl crate::Readable for FULLDUPLEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [full_duplex::W](W) writer structure"]
impl crate::Writable for FULLDUPLEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FullDuplex to value 0x01"]
impl crate::Resettable for FULLDUPLEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
