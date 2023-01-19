#[doc = "Register `UART_DATA` reader"]
pub struct R(crate::R<UART_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BYTE` reader - "]
pub type BYTE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn byte(&self) -> BYTE_R {
        BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX read data for polling or interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_data](index.html) module"]
pub struct UART_DATA_SPEC;
impl crate::RegisterSpec for UART_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_data::R](R) reader structure"]
impl crate::Readable for UART_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_DATA to value 0"]
impl crate::Resettable for UART_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
