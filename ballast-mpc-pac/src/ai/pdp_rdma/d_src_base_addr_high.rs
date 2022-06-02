#[doc = "Register `D_SRC_BASE_ADDR_HIGH` reader"]
pub struct R(crate::R<D_SRC_BASE_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SRC_BASE_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SRC_BASE_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SRC_BASE_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC_BASE_ADDR_HIGH` reader - "]
pub struct SRC_BASE_ADDR_HIGH_R(crate::FieldReader<u32>);
impl SRC_BASE_ADDR_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SRC_BASE_ADDR_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_BASE_ADDR_HIGH_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_base_addr_high(&self) -> SRC_BASE_ADDR_HIGH_R {
        SRC_BASE_ADDR_HIGH_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_src_base_addr_high](index.html) module"]
pub struct D_SRC_BASE_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for D_SRC_BASE_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_src_base_addr_high::R](R) reader structure"]
impl crate::Readable for D_SRC_BASE_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_SRC_BASE_ADDR_HIGH to value 0"]
impl crate::Resettable for D_SRC_BASE_ADDR_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}