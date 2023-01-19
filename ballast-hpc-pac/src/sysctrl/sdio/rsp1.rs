#[doc = "Register `RSP1` reader"]
pub struct R(crate::R<RSP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE_WORD1` reader - "]
pub type RESPONSE_WORD1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word1(&self) -> RESPONSE_WORD1_R {
        RESPONSE_WORD1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp1](index.html) module"]
pub struct RSP1_SPEC;
impl crate::RegisterSpec for RSP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsp1::R](R) reader structure"]
impl crate::Readable for RSP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSP1 to value 0"]
impl crate::Resettable for RSP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
