#[doc = "Register `SDP_BN_THROUGHPUT_0` reader"]
pub struct R(crate::R<SDP_BN_THROUGHPUT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDP_BN_THROUGHPUT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDP_BN_THROUGHPUT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDP_BN_THROUGHPUT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDP_BN_THROUGHPUT` reader - "]
pub type SDP_BN_THROUGHPUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sdp_bn_throughput(&self) -> SDP_BN_THROUGHPUT_R {
        SDP_BN_THROUGHPUT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdp_bn_throughput_0](index.html) module"]
pub struct SDP_BN_THROUGHPUT_0_SPEC;
impl crate::RegisterSpec for SDP_BN_THROUGHPUT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdp_bn_throughput_0::R](R) reader structure"]
impl crate::Readable for SDP_BN_THROUGHPUT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDP_BN_THROUGHPUT_0 to value 0x01"]
impl crate::Resettable for SDP_BN_THROUGHPUT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
