#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `cluster_count` reader - Number of Clusters"]
pub type CLUSTER_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `core_count` reader - Number of Cores"]
pub type CORE_COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Clusters"]
    #[inline(always)]
    pub fn cluster_count(&self) -> CLUSTER_COUNT_R {
        CLUSTER_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of Cores"]
    #[inline(always)]
    pub fn core_count(&self) -> CORE_COUNT_R {
        CORE_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
