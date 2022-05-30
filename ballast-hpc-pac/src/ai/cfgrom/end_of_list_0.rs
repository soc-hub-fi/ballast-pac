#[doc = "Register `END_OF_LIST_0` reader"]
pub struct R(crate::R<END_OF_LIST_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<END_OF_LIST_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<END_OF_LIST_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<END_OF_LIST_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `END_OF_LIST` reader - "]
pub struct END_OF_LIST_R(crate::FieldReader<u32>);
impl END_OF_LIST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        END_OF_LIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_OF_LIST_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn end_of_list(&self) -> END_OF_LIST_R {
        END_OF_LIST_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [end_of_list_0](index.html) module"]
pub struct END_OF_LIST_0_SPEC;
impl crate::RegisterSpec for END_OF_LIST_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [end_of_list_0::R](R) reader structure"]
impl crate::Readable for END_OF_LIST_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets END_OF_LIST_0 to value 0"]
impl crate::Resettable for END_OF_LIST_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
