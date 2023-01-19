#[doc = "Register `SDP_RDMA_CAP_COMPAT_0` reader"]
pub struct R(crate::R<SDP_RDMA_CAP_COMPAT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDP_RDMA_CAP_COMPAT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDP_RDMA_CAP_COMPAT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDP_RDMA_CAP_COMPAT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDP_RDMA_CAP_COMPAT` reader - "]
pub type SDP_RDMA_CAP_COMPAT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sdp_rdma_cap_compat(&self) -> SDP_RDMA_CAP_COMPAT_R {
        SDP_RDMA_CAP_COMPAT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdp_rdma_cap_compat_0](index.html) module"]
pub struct SDP_RDMA_CAP_COMPAT_0_SPEC;
impl crate::RegisterSpec for SDP_RDMA_CAP_COMPAT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdp_rdma_cap_compat_0::R](R) reader structure"]
impl crate::Readable for SDP_RDMA_CAP_COMPAT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDP_RDMA_CAP_COMPAT_0 to value 0"]
impl crate::Resettable for SDP_RDMA_CAP_COMPAT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
