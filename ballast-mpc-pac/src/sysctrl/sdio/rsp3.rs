#[doc = "Register `RSP3` reader"]
pub struct R(crate::R<RSP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPONSE_WORD3` reader - "]
pub struct RESPONSE_WORD3_R(crate::FieldReader<u32, u32>);
impl RESPONSE_WORD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RESPONSE_WORD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_WORD3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word3(&self) -> RESPONSE_WORD3_R {
        RESPONSE_WORD3_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp3](index.html) module"]
pub struct RSP3_SPEC;
impl crate::RegisterSpec for RSP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsp3::R](R) reader structure"]
impl crate::Readable for RSP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSP3 to value 0"]
impl crate::Resettable for RSP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
