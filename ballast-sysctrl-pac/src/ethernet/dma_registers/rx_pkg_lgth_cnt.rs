#[doc = "Register `rx_pkg_lgth_cnt` reader"]
pub struct R(crate::R<RX_PKG_LGTH_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PKG_LGTH_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PKG_LGTH_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PKG_LGTH_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_pkg_lgth_cnt` reader - "]
pub type RX_PKG_LGTH_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rx_pkg_lgth_cnt(&self) -> RX_PKG_LGTH_CNT_R {
        RX_PKG_LGTH_CNT_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Read-only register denoting how many packet lengths are available or yet to be processed in the corresponding FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_pkg_lgth_cnt](index.html) module"]
pub struct RX_PKG_LGTH_CNT_SPEC;
impl crate::RegisterSpec for RX_PKG_LGTH_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_pkg_lgth_cnt::R](R) reader structure"]
impl crate::Readable for RX_PKG_LGTH_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx_pkg_lgth_cnt to value 0"]
impl crate::Resettable for RX_PKG_LGTH_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
