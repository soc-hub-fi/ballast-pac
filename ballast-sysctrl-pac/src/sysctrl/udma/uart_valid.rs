#[doc = "Register `UART_VALID` reader"]
pub struct R(crate::R<UART_VALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_VALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_VALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_VALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY` reader - Used only in RX polling method to indicate data is ready for read: - 1'b0: Data is not ready to read - 1'b1: Data is ready to read"]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Used only in RX polling method to indicate data is ready for read: - 1'b0: Data is not ready to read - 1'b1: Data is ready to read"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "uDMA UART Read polling data valid flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_valid](index.html) module"]
pub struct UART_VALID_SPEC;
impl crate::RegisterSpec for UART_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_valid::R](R) reader structure"]
impl crate::Readable for UART_VALID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_VALID to value 0"]
impl crate::Resettable for UART_VALID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
