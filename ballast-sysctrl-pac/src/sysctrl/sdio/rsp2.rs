#[doc = "Register `RSP2` reader"]
pub struct R(crate::R<RSP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE_WORD2` reader - "]
pub type RESPONSE_WORD2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word2(&self) -> RESPONSE_WORD2_R {
        RESPONSE_WORD2_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp2](index.html) module"]
pub struct RSP2_SPEC;
impl crate::RegisterSpec for RSP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsp2::R](R) reader structure"]
impl crate::Readable for RSP2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSP2 to value 0"]
impl crate::Resettable for RSP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
