#[doc = "Register `CDMA_BASE_CBUF_BANK_NUM_0` reader"]
pub struct R(crate::R<CDMA_BASE_CBUF_BANK_NUM_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDMA_BASE_CBUF_BANK_NUM_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDMA_BASE_CBUF_BANK_NUM_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDMA_BASE_CBUF_BANK_NUM_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CDMA_BASE_CBUF_BANK_NUM` reader - "]
pub type CDMA_BASE_CBUF_BANK_NUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cdma_base_cbuf_bank_num(&self) -> CDMA_BASE_CBUF_BANK_NUM_R {
        CDMA_BASE_CBUF_BANK_NUM_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdma_base_cbuf_bank_num_0](index.html) module"]
pub struct CDMA_BASE_CBUF_BANK_NUM_0_SPEC;
impl crate::RegisterSpec for CDMA_BASE_CBUF_BANK_NUM_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdma_base_cbuf_bank_num_0::R](R) reader structure"]
impl crate::Readable for CDMA_BASE_CBUF_BANK_NUM_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDMA_BASE_CBUF_BANK_NUM_0 to value 0x20"]
impl crate::Resettable for CDMA_BASE_CBUF_BANK_NUM_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
