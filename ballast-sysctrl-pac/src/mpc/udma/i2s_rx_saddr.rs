#[doc = "Register `I2S_RX_SADDR` reader"]
pub struct R(crate::R<I2S_RX_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_RX_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_RX_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_RX_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_RX_SADDR` writer"]
pub struct W(crate::W<I2S_RX_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_RX_SADDR_SPEC>;
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
impl From<crate::W<I2S_RX_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_RX_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SADDR` reader - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub struct RX_SADDR_R(crate::FieldReader<u16, u16>);
impl RX_SADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SADDR` writer - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub struct RX_SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn rx_saddr(&self) -> RX_SADDR_R {
        RX_SADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn rx_saddr(&mut self) -> RX_SADDR_W {
        RX_SADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Channel0 I2S uDMA transfer address of associated buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_saddr](index.html) module"]
pub struct I2S_RX_SADDR_SPEC;
impl crate::RegisterSpec for I2S_RX_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_rx_saddr::R](R) reader structure"]
impl crate::Readable for I2S_RX_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_rx_saddr::W](W) writer structure"]
impl crate::Writable for I2S_RX_SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_RX_SADDR to value 0"]
impl crate::Resettable for I2S_RX_SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
