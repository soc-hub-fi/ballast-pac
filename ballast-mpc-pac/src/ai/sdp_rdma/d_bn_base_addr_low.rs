#[doc = "Register `D_BN_BASE_ADDR_LOW` reader"]
pub struct R(crate::R<D_BN_BASE_ADDR_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_BN_BASE_ADDR_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_BN_BASE_ADDR_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_BN_BASE_ADDR_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BN_BASE_ADDR_LOW` reader - "]
pub struct BN_BASE_ADDR_LOW_R(crate::FieldReader<u32, u32>);
impl BN_BASE_ADDR_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BN_BASE_ADDR_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BN_BASE_ADDR_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bn_base_addr_low(&self) -> BN_BASE_ADDR_LOW_R {
        BN_BASE_ADDR_LOW_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_bn_base_addr_low](index.html) module"]
pub struct D_BN_BASE_ADDR_LOW_SPEC;
impl crate::RegisterSpec for D_BN_BASE_ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_bn_base_addr_low::R](R) reader structure"]
impl crate::Readable for D_BN_BASE_ADDR_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_BN_BASE_ADDR_LOW to value 0"]
impl crate::Resettable for D_BN_BASE_ADDR_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
