#[doc = "Register `rx_pkg_lgth` reader"]
pub struct R(crate::R<RX_PKG_LGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PKG_LGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PKG_LGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PKG_LGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_pkg_lgth` reader - "]
pub type RX_PKG_LGTH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pkg_lgth(&self) -> RX_PKG_LGTH_R {
        RX_PKG_LGTH_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Data access to the package length FIFO. Length in bytes. rx_pkg_lgth is read-only. The DMA write channel pushes packet lengths to the FIFO after the corresponding AXI transfers for the packet have finished. Reading this register when the associated FIFO is not empty removes the first-written piece of data from the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_pkg_lgth](index.html) module"]
pub struct RX_PKG_LGTH_SPEC;
impl crate::RegisterSpec for RX_PKG_LGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_pkg_lgth::R](R) reader structure"]
impl crate::Readable for RX_PKG_LGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx_pkg_lgth to value 0"]
impl crate::Resettable for RX_PKG_LGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
