#[doc = "Register `FIFO_DATA` reader"]
pub struct R(crate::R<FIFO_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_DATA` reader - This is a read-only register that contain the first valid value of the FIFO"]
pub struct FIFO_DATA_R(crate::FieldReader<u32, u32>);
impl FIFO_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FIFO_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This is a read-only register that contain the first valid value of the FIFO"]
    #[inline(always)]
    pub fn fifo_data(&self) -> FIFO_DATA_R {
        FIFO_DATA_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_data](index.html) module"]
pub struct FIFO_DATA_SPEC;
impl crate::RegisterSpec for FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_data::R](R) reader structure"]
impl crate::Readable for FIFO_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FIFO_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
