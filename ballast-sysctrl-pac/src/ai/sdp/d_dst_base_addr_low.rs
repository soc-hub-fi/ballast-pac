#[doc = "Register `D_DST_BASE_ADDR_LOW` reader"]
pub struct R(crate::R<D_DST_BASE_ADDR_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DST_BASE_ADDR_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DST_BASE_ADDR_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DST_BASE_ADDR_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DST_BASE_ADDR_LOW` reader - "]
pub type DST_BASE_ADDR_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_base_addr_low(&self) -> DST_BASE_ADDR_LOW_R {
        DST_BASE_ADDR_LOW_R::new(self.bits)
    }
}
#[doc = "Lower 32bits of output data address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dst_base_addr_low](index.html) module"]
pub struct D_DST_BASE_ADDR_LOW_SPEC;
impl crate::RegisterSpec for D_DST_BASE_ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dst_base_addr_low::R](R) reader structure"]
impl crate::Readable for D_DST_BASE_ADDR_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DST_BASE_ADDR_LOW to value 0"]
impl crate::Resettable for D_DST_BASE_ADDR_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
