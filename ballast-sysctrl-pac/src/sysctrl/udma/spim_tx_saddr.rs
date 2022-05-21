#[doc = "Register `SPIM_TX_SADDR` reader"]
pub struct R(crate::R<SPIM_TX_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIM_TX_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIM_TX_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIM_TX_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIM_TX_SADDR` writer"]
pub struct W(crate::W<SPIM_TX_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIM_TX_SADDR_SPEC>;
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
impl From<crate::W<SPIM_TX_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIM_TX_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SADDR` reader - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub struct TX_SADDR_R(crate::FieldReader<u32, u32>);
impl TX_SADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TX_SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SADDR` writer - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub struct TX_SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn tx_saddr(&self) -> TX_SADDR_R {
        TX_SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn tx_saddr(&mut self) -> TX_SADDR_W {
        TX_SADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX SPI uDMA transfer address of associated buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spim_tx_saddr](index.html) module"]
pub struct SPIM_TX_SADDR_SPEC;
impl crate::RegisterSpec for SPIM_TX_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spim_tx_saddr::R](R) reader structure"]
impl crate::Readable for SPIM_TX_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spim_tx_saddr::W](W) writer structure"]
impl crate::Writable for SPIM_TX_SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIM_TX_SADDR to value 0"]
impl crate::Resettable for SPIM_TX_SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
