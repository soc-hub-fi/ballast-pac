#[doc = "Register `SDP_BASE_FEATURE_TYPES_0` reader"]
pub struct R(crate::R<SDP_BASE_FEATURE_TYPES_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDP_BASE_FEATURE_TYPES_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDP_BASE_FEATURE_TYPES_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDP_BASE_FEATURE_TYPES_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDP_BASE_FEATURE_TYPES` reader - "]
pub type SDP_BASE_FEATURE_TYPES_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn sdp_base_feature_types(&self) -> SDP_BASE_FEATURE_TYPES_R {
        SDP_BASE_FEATURE_TYPES_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdp_base_feature_types_0](index.html) module"]
pub struct SDP_BASE_FEATURE_TYPES_0_SPEC;
impl crate::RegisterSpec for SDP_BASE_FEATURE_TYPES_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdp_base_feature_types_0::R](R) reader structure"]
impl crate::Readable for SDP_BASE_FEATURE_TYPES_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDP_BASE_FEATURE_TYPES_0 to value 0x10"]
impl crate::Resettable for SDP_BASE_FEATURE_TYPES_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
