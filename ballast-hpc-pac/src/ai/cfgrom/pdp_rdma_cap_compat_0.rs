#[doc = "Register `PDP_RDMA_CAP_COMPAT_0` reader"]
pub struct R(crate::R<PDP_RDMA_CAP_COMPAT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDP_RDMA_CAP_COMPAT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDP_RDMA_CAP_COMPAT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDP_RDMA_CAP_COMPAT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDP_RDMA_CAP_COMPAT` reader - "]
pub type PDP_RDMA_CAP_COMPAT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pdp_rdma_cap_compat(&self) -> PDP_RDMA_CAP_COMPAT_R {
        PDP_RDMA_CAP_COMPAT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdp_rdma_cap_compat_0](index.html) module"]
pub struct PDP_RDMA_CAP_COMPAT_0_SPEC;
impl crate::RegisterSpec for PDP_RDMA_CAP_COMPAT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdp_rdma_cap_compat_0::R](R) reader structure"]
impl crate::Readable for PDP_RDMA_CAP_COMPAT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDP_RDMA_CAP_COMPAT_0 to value 0"]
impl crate::Resettable for PDP_RDMA_CAP_COMPAT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
