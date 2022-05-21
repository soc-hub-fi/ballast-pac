#[doc = "Register `ERR7` reader"]
pub struct R(crate::R<ERR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR7` reader - "]
pub struct ERR7_R(crate::FieldReader<u32, u32>);
impl ERR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(self.bits)
    }
}
#[doc = "Events 224-255 event queue overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err7](index.html) module"]
pub struct ERR7_SPEC;
impl crate::RegisterSpec for ERR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err7::R](R) reader structure"]
impl crate::Readable for ERR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR7 to value 0"]
impl crate::Resettable for ERR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
