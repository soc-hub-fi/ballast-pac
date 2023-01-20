#[doc = "Register `SPIM_RX_SADDR` reader"]
pub struct R(crate::R<SPIM_RX_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIM_RX_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIM_RX_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIM_RX_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIM_RX_SADDR` writer"]
pub struct W(crate::W<SPIM_RX_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIM_RX_SADDR_SPEC>;
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
impl From<crate::W<SPIM_RX_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIM_RX_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SADDR` reader - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub type RX_SADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RX_SADDR` writer - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub type RX_SADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPIM_RX_SADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn rx_saddr(&self) -> RX_SADDR_R {
        RX_SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    #[must_use]
    pub fn rx_saddr(&mut self) -> RX_SADDR_W<0> {
        RX_SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX SPI uDMA transfer address of associated buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spim_rx_saddr](index.html) module"]
pub struct SPIM_RX_SADDR_SPEC;
impl crate::RegisterSpec for SPIM_RX_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spim_rx_saddr::R](R) reader structure"]
impl crate::Readable for SPIM_RX_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spim_rx_saddr::W](W) writer structure"]
impl crate::Writable for SPIM_RX_SADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIM_RX_SADDR to value 0"]
impl crate::Resettable for SPIM_RX_SADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
