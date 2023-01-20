#[doc = "Register `interface_version` reader"]
pub struct R(crate::R<INTERFACE_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERFACE_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERFACE_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERFACE_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `interface_version` reader - "]
pub type INTERFACE_VERSION_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interface_version(&self) -> INTERFACE_VERSION_R {
        INTERFACE_VERSION_R::new(self.bits)
    }
}
#[doc = "Interface version (0x2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interface_version](index.html) module"]
pub struct INTERFACE_VERSION_SPEC;
impl crate::RegisterSpec for INTERFACE_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interface_version::R](R) reader structure"]
impl crate::Readable for INTERFACE_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets interface_version to value 0"]
impl crate::Resettable for INTERFACE_VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
