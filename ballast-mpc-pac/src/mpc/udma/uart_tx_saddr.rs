#[doc = "Register `UART_TX_SADDR` reader"]
pub struct R(crate::R<UART_TX_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TX_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TX_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TX_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TX_SADDR` writer"]
pub struct W(crate::W<UART_TX_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TX_SADDR_SPEC>;
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
impl From<crate::W<UART_TX_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TX_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SADDR` reader - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub type TX_SADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_SADDR` writer - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub type TX_SADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_TX_SADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    pub fn tx_saddr(&self) -> TX_SADDR_R {
        TX_SADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    #[must_use]
    pub fn tx_saddr(&mut self) -> TX_SADDR_W<0> {
        TX_SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA TX UART buffer base address configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tx_saddr](index.html) module"]
pub struct UART_TX_SADDR_SPEC;
impl crate::RegisterSpec for UART_TX_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tx_saddr::R](R) reader structure"]
impl crate::Readable for UART_TX_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tx_saddr::W](W) writer structure"]
impl crate::Writable for UART_TX_SADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_TX_SADDR to value 0"]
impl crate::Resettable for UART_TX_SADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
