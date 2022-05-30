#[doc = "Register `D_DST_BASE_ADDR_HIGH` reader"]
pub struct R(crate::R<D_DST_BASE_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DST_BASE_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DST_BASE_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DST_BASE_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DST_BASE_ADDR_HIGH` reader - "]
pub struct DST_BASE_ADDR_HIGH_R(crate::FieldReader<u32>);
impl DST_BASE_ADDR_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DST_BASE_ADDR_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_BASE_ADDR_HIGH_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_base_addr_high(&self) -> DST_BASE_ADDR_HIGH_R {
        DST_BASE_ADDR_HIGH_R::new(self.bits)
    }
}
#[doc = "Higher 32bits of output data address when axi awaddr is 64bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dst_base_addr_high](index.html) module"]
pub struct D_DST_BASE_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for D_DST_BASE_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dst_base_addr_high::R](R) reader structure"]
impl crate::Readable for D_DST_BASE_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DST_BASE_ADDR_HIGH to value 0"]
impl crate::Resettable for D_DST_BASE_ADDR_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
