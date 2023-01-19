#[doc = "Register `CDP_RDMA_BASE_CDP_ID_0` reader"]
pub struct R(crate::R<CDP_RDMA_BASE_CDP_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDP_RDMA_BASE_CDP_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDP_RDMA_BASE_CDP_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDP_RDMA_BASE_CDP_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CDP_RDMA_BASE_CDP_ID` reader - "]
pub type CDP_RDMA_BASE_CDP_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cdp_rdma_base_cdp_id(&self) -> CDP_RDMA_BASE_CDP_ID_R {
        CDP_RDMA_BASE_CDP_ID_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdp_rdma_base_cdp_id_0](index.html) module"]
pub struct CDP_RDMA_BASE_CDP_ID_0_SPEC;
impl crate::RegisterSpec for CDP_RDMA_BASE_CDP_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdp_rdma_base_cdp_id_0::R](R) reader structure"]
impl crate::Readable for CDP_RDMA_BASE_CDP_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDP_RDMA_BASE_CDP_ID_0 to value 0x0d"]
impl crate::Resettable for CDP_RDMA_BASE_CDP_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
