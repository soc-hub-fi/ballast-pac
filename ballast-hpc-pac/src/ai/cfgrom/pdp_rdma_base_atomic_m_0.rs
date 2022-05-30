#[doc = "Register `PDP_RDMA_BASE_ATOMIC_M_0` reader"]
pub struct R(crate::R<PDP_RDMA_BASE_ATOMIC_M_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDP_RDMA_BASE_ATOMIC_M_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDP_RDMA_BASE_ATOMIC_M_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDP_RDMA_BASE_ATOMIC_M_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDP_RDMA_BASE_ATOMIC_M` reader - "]
pub struct PDP_RDMA_BASE_ATOMIC_M_R(crate::FieldReader<u32>);
impl PDP_RDMA_BASE_ATOMIC_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PDP_RDMA_BASE_ATOMIC_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDP_RDMA_BASE_ATOMIC_M_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pdp_rdma_base_atomic_m(&self) -> PDP_RDMA_BASE_ATOMIC_M_R {
        PDP_RDMA_BASE_ATOMIC_M_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdp_rdma_base_atomic_m_0](index.html) module"]
pub struct PDP_RDMA_BASE_ATOMIC_M_0_SPEC;
impl crate::RegisterSpec for PDP_RDMA_BASE_ATOMIC_M_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdp_rdma_base_atomic_m_0::R](R) reader structure"]
impl crate::Readable for PDP_RDMA_BASE_ATOMIC_M_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDP_RDMA_BASE_ATOMIC_M_0 to value 0x08"]
impl crate::Resettable for PDP_RDMA_BASE_ATOMIC_M_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
