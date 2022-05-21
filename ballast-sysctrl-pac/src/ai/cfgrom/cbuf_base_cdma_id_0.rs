#[doc = "Register `CBUF_BASE_CDMA_ID_0` reader"]
pub struct R(crate::R<CBUF_BASE_CDMA_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBUF_BASE_CDMA_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBUF_BASE_CDMA_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBUF_BASE_CDMA_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CBUF_BASE_CDMA_ID` reader - "]
pub struct CBUF_BASE_CDMA_ID_R(crate::FieldReader<u32, u32>);
impl CBUF_BASE_CDMA_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CBUF_BASE_CDMA_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBUF_BASE_CDMA_ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbuf_base_cdma_id(&self) -> CBUF_BASE_CDMA_ID_R {
        CBUF_BASE_CDMA_ID_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbuf_base_cdma_id_0](index.html) module"]
pub struct CBUF_BASE_CDMA_ID_0_SPEC;
impl crate::RegisterSpec for CBUF_BASE_CDMA_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbuf_base_cdma_id_0::R](R) reader structure"]
impl crate::Readable for CBUF_BASE_CDMA_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CBUF_BASE_CDMA_ID_0 to value 0x03"]
impl crate::Resettable for CBUF_BASE_CDMA_ID_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
