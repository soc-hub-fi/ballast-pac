#[doc = "Register `tx_pkg_lgth_cnt` reader"]
pub struct R(crate::R<TX_PKG_LGTH_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PKG_LGTH_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PKG_LGTH_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PKG_LGTH_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tx_pkg_lgth_cnt` reader - "]
pub type TX_PKG_LGTH_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn tx_pkg_lgth_cnt(&self) -> TX_PKG_LGTH_CNT_R {
        TX_PKG_LGTH_CNT_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Read-only registers denoting how many packet lengths are available or yet to be processed in the corresponding FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pkg_lgth_cnt](index.html) module"]
pub struct TX_PKG_LGTH_CNT_SPEC;
impl crate::RegisterSpec for TX_PKG_LGTH_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pkg_lgth_cnt::R](R) reader structure"]
impl crate::Readable for TX_PKG_LGTH_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tx_pkg_lgth_cnt to value 0"]
impl crate::Resettable for TX_PKG_LGTH_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
