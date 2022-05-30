#[doc = "Register `device_id` reader"]
pub struct R(crate::R<DEVICE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `device_id` reader - device id"]
pub struct DEVICE_ID_R(crate::FieldReader<u32>);
impl DEVICE_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DEVICE_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICE_ID_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - device id"]
    #[inline(always)]
    pub fn device_id(&self) -> DEVICE_ID_R {
        DEVICE_ID_R::new(self.bits)
    }
}
#[doc = "Device ID (unimplemented)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id](index.html) module"]
pub struct DEVICE_ID_SPEC;
impl crate::RegisterSpec for DEVICE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_id::R](R) reader structure"]
impl crate::Readable for DEVICE_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets device_id to value 0"]
impl crate::Resettable for DEVICE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
