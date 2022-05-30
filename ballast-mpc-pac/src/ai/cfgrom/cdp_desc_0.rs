#[doc = "Register `CDP_DESC_0` reader"]
pub struct R(crate::R<CDP_DESC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDP_DESC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDP_DESC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDP_DESC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CDP_DESC` reader - "]
pub struct CDP_DESC_R(crate::FieldReader<u32>);
impl CDP_DESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CDP_DESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDP_DESC_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cdp_desc(&self) -> CDP_DESC_R {
        CDP_DESC_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdp_desc_0](index.html) module"]
pub struct CDP_DESC_0_SPEC;
impl crate::RegisterSpec for CDP_DESC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdp_desc_0::R](R) reader structure"]
impl crate::Readable for CDP_DESC_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDP_DESC_0 to value 0x0010_000d"]
impl crate::Resettable for CDP_DESC_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_000d
    }
}
