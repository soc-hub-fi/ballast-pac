#[doc = "Register `HW_VERSION` reader"]
pub struct R(crate::R<HW_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAJOR` reader - "]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR` reader - "]
pub type MINOR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "HW version of NVDLA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_version](index.html) module"]
pub struct HW_VERSION_SPEC;
impl crate::RegisterSpec for HW_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_version::R](R) reader structure"]
impl crate::Readable for HW_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HW_VERSION to value 0x3030_0031"]
impl crate::Resettable for HW_VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x3030_0031;
}
