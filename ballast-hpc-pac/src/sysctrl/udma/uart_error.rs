#[doc = "Register `UART_ERROR` reader"]
pub struct R(crate::R<UART_ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_ERR_OVERFLOW` reader - RX overflow error status flag: - 1'b0: no error - 1'b1: RX overflow error occurred"]
pub struct RX_ERR_OVERFLOW_R(crate::FieldReader<bool>);
impl RX_ERR_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ERR_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERR_OVERFLOW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERR_PARITY` reader - RX parity error status flag: - 1'b0: no error - 1'b1: RX parity error occurred"]
pub struct RX_ERR_PARITY_R(crate::FieldReader<bool>);
impl RX_ERR_PARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ERR_PARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERR_PARITY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RX overflow error status flag: - 1'b0: no error - 1'b1: RX overflow error occurred"]
    #[inline(always)]
    pub fn rx_err_overflow(&self) -> RX_ERR_OVERFLOW_R {
        RX_ERR_OVERFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX parity error status flag: - 1'b0: no error - 1'b1: RX parity error occurred"]
    #[inline(always)]
    pub fn rx_err_parity(&self) -> RX_ERR_PARITY_R {
        RX_ERR_PARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "uDMA UART Error status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_error](index.html) module"]
pub struct UART_ERROR_SPEC;
impl crate::RegisterSpec for UART_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_error::R](R) reader structure"]
impl crate::Readable for UART_ERROR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_ERROR to value 0"]
impl crate::Resettable for UART_ERROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
