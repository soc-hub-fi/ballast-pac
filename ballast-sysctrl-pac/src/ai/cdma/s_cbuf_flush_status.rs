#[doc = "Register `S_CBUF_FLUSH_STATUS` reader"]
pub struct R(crate::R<S_CBUF_FLUSH_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_CBUF_FLUSH_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_CBUF_FLUSH_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_CBUF_FLUSH_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLUSH_DONE` reader - "]
pub type FLUSH_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flush_done(&self) -> FLUSH_DONE_R {
        FLUSH_DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Indicates whether CBUF flush is finished after reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_cbuf_flush_status](index.html) module"]
pub struct S_CBUF_FLUSH_STATUS_SPEC;
impl crate::RegisterSpec for S_CBUF_FLUSH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_cbuf_flush_status::R](R) reader structure"]
impl crate::Readable for S_CBUF_FLUSH_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_CBUF_FLUSH_STATUS to value 0"]
impl crate::Resettable for S_CBUF_FLUSH_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
