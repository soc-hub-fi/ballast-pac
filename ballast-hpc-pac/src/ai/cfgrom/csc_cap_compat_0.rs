#[doc = "Register `CSC_CAP_COMPAT_0` reader"]
pub struct R(crate::R<CSC_CAP_COMPAT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSC_CAP_COMPAT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSC_CAP_COMPAT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSC_CAP_COMPAT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSC_CAP_COMPAT` reader - "]
pub struct CSC_CAP_COMPAT_R(crate::FieldReader<u32>);
impl CSC_CAP_COMPAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CSC_CAP_COMPAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_CAP_COMPAT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn csc_cap_compat(&self) -> CSC_CAP_COMPAT_R {
        CSC_CAP_COMPAT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc_cap_compat_0](index.html) module"]
pub struct CSC_CAP_COMPAT_0_SPEC;
impl crate::RegisterSpec for CSC_CAP_COMPAT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csc_cap_compat_0::R](R) reader structure"]
impl crate::Readable for CSC_CAP_COMPAT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSC_CAP_COMPAT_0 to value 0x10"]
impl crate::Resettable for CSC_CAP_COMPAT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
