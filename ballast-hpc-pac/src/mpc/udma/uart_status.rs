#[doc = "Register `UART_STATUS` reader"]
pub struct R(crate::R<UART_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_BUSY` reader - TX busy status flag: - 1'b0: no TX transfer on-going - 1'b1: TX transfer on-going"]
pub struct TX_BUSY_R(crate::FieldReader<bool>);
impl TX_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BUSY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BUSY` reader - RX busy status flag: - 1'b0: no RX transfer on-going - 1'b1: RX transfer on-going"]
pub struct RX_BUSY_R(crate::FieldReader<bool>);
impl RX_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BUSY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TX busy status flag: - 1'b0: no TX transfer on-going - 1'b1: TX transfer on-going"]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX busy status flag: - 1'b0: no RX transfer on-going - 1'b1: RX transfer on-going"]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "uDMA UART status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](index.html) module"]
pub struct UART_STATUS_SPEC;
impl crate::RegisterSpec for UART_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_status::R](R) reader structure"]
impl crate::Readable for UART_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_STATUS to value 0"]
impl crate::Resettable for UART_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
