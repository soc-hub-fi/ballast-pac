#[doc = "Register `UART_RX_SIZE` reader"]
pub struct R(crate::R<UART_RX_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RX_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RX_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RX_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_RX_SIZE` writer"]
pub struct W(crate::W<UART_RX_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RX_SIZE_SPEC>;
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
impl From<crate::W<UART_RX_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RX_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SIZE` reader - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type RX_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RX_SIZE` writer - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type RX_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_RX_SIZE_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RX_SIZE_W<0> {
        RX_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA RX UART buffer size configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_size](index.html) module"]
pub struct UART_RX_SIZE_SPEC;
impl crate::RegisterSpec for UART_RX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rx_size::R](R) reader structure"]
impl crate::Readable for UART_RX_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rx_size::W](W) writer structure"]
impl crate::Writable for UART_RX_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_RX_SIZE to value 0"]
impl crate::Resettable for UART_RX_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
