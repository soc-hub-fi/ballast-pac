#[doc = "Register `tx_pkg_lgth` writer"]
pub struct W(crate::W<TX_PKG_LGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PKG_LGTH_SPEC>;
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
impl From<crate::W<TX_PKG_LGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PKG_LGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_pkg_lgth` writer - "]
pub type TX_PKG_LGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_PKG_LGTH_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pkg_lgth(&mut self) -> TX_PKG_LGTH_W<0> {
        TX_PKG_LGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data access to the package length FIFO. Packet lengths in bytes. tx_pkg_lgth is write-only. To issue a packet to be sent by the MAC, the host processor writes the ethernet frame data to the ring buffer, starting from a 4-byte aligned address immediately after the end of the previous packet, and writes the length of the packet to tx_pkg_lgth, in this order.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pkg_lgth](index.html) module"]
pub struct TX_PKG_LGTH_SPEC;
impl crate::RegisterSpec for TX_PKG_LGTH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_pkg_lgth::W](W) writer structure"]
impl crate::Writable for TX_PKG_LGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tx_pkg_lgth to value 0"]
impl crate::Resettable for TX_PKG_LGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
