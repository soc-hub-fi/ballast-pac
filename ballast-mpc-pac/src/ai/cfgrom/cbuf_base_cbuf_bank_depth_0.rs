#[doc = "Register `CBUF_BASE_CBUF_BANK_DEPTH_0` reader"]
pub struct R(crate::R<CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CBUF_BASE_CBUF_BANK_DEPTH` reader - "]
pub type CBUF_BASE_CBUF_BANK_DEPTH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbuf_base_cbuf_bank_depth(&self) -> CBUF_BASE_CBUF_BANK_DEPTH_R {
        CBUF_BASE_CBUF_BANK_DEPTH_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbuf_base_cbuf_bank_depth_0](index.html) module"]
pub struct CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC;
impl crate::RegisterSpec for CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbuf_base_cbuf_bank_depth_0::R](R) reader structure"]
impl crate::Readable for CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CBUF_BASE_CBUF_BANK_DEPTH_0 to value 0x0200"]
impl crate::Resettable for CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
