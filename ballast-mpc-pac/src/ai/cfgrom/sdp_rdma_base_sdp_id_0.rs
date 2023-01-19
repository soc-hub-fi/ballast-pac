#[doc = "Register `SDP_RDMA_BASE_SDP_ID_0` reader"]
pub struct R(crate::R<SDP_RDMA_BASE_SDP_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDP_RDMA_BASE_SDP_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDP_RDMA_BASE_SDP_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDP_RDMA_BASE_SDP_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDP_RDMA_BASE_SDP_ID` reader - "]
pub type SDP_RDMA_BASE_SDP_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sdp_rdma_base_sdp_id(&self) -> SDP_RDMA_BASE_SDP_ID_R {
        SDP_RDMA_BASE_SDP_ID_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdp_rdma_base_sdp_id_0](index.html) module"]
pub struct SDP_RDMA_BASE_SDP_ID_0_SPEC;
impl crate::RegisterSpec for SDP_RDMA_BASE_SDP_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdp_rdma_base_sdp_id_0::R](R) reader structure"]
impl crate::Readable for SDP_RDMA_BASE_SDP_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDP_RDMA_BASE_SDP_ID_0 to value 0x09"]
impl crate::Resettable for SDP_RDMA_BASE_SDP_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
