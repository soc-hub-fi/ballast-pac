#[doc = "Register `CS_RO` reader"]
pub struct R(crate::R<CS_RO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_RO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_RO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_RO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CS_RO` reader - "]
pub struct CS_RO_R(crate::FieldReader<u32, u32>);
impl CS_RO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CS_RO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_RO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cs_ro(&self) -> CS_RO_R {
        CS_RO_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs_ro](index.html) module"]
pub struct CS_RO_SPEC;
impl crate::RegisterSpec for CS_RO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs_ro::R](R) reader structure"]
impl crate::Readable for CS_RO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CS_RO to value 0"]
impl crate::Resettable for CS_RO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
