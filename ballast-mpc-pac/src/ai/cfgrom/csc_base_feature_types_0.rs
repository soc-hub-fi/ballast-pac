#[doc = "Register `CSC_BASE_FEATURE_TYPES_0` reader"]
pub struct R(crate::R<CSC_BASE_FEATURE_TYPES_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSC_BASE_FEATURE_TYPES_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSC_BASE_FEATURE_TYPES_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSC_BASE_FEATURE_TYPES_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSC_BASE_FEATURE_TYPES` reader - "]
pub struct CSC_BASE_FEATURE_TYPES_R(crate::FieldReader<u16, u16>);
impl CSC_BASE_FEATURE_TYPES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CSC_BASE_FEATURE_TYPES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_BASE_FEATURE_TYPES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn csc_base_feature_types(&self) -> CSC_BASE_FEATURE_TYPES_R {
        CSC_BASE_FEATURE_TYPES_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc_base_feature_types_0](index.html) module"]
pub struct CSC_BASE_FEATURE_TYPES_0_SPEC;
impl crate::RegisterSpec for CSC_BASE_FEATURE_TYPES_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csc_base_feature_types_0::R](R) reader structure"]
impl crate::Readable for CSC_BASE_FEATURE_TYPES_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSC_BASE_FEATURE_TYPES_0 to value 0x10"]
impl crate::Resettable for CSC_BASE_FEATURE_TYPES_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
