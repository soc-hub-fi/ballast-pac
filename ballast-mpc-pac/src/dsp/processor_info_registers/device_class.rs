#[doc = "Register `device_class` reader"]
pub struct R(crate::R<DEVICE_CLASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_CLASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_CLASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_CLASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `device_class` reader - "]
pub type DEVICE_CLASS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn device_class(&self) -> DEVICE_CLASS_R {
        DEVICE_CLASS_R::new(self.bits)
    }
}
#[doc = "Device class (0x774)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_class](index.html) module"]
pub struct DEVICE_CLASS_SPEC;
impl crate::RegisterSpec for DEVICE_CLASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_class::R](R) reader structure"]
impl crate::Readable for DEVICE_CLASS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets device_class to value 0"]
impl crate::Resettable for DEVICE_CLASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
