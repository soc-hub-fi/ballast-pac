#[doc = "Register `D_BATCH_NUMBER` reader"]
pub struct R(crate::R<D_BATCH_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_BATCH_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_BATCH_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_BATCH_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BATCHES` reader - "]
pub struct BATCHES_R(crate::FieldReader<u8, u8>);
impl BATCHES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BATCHES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BATCHES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn batches(&self) -> BATCHES_R {
        BATCHES_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Number of batches\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_batch_number](index.html) module"]
pub struct D_BATCH_NUMBER_SPEC;
impl crate::RegisterSpec for D_BATCH_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_batch_number::R](R) reader structure"]
impl crate::Readable for D_BATCH_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_BATCH_NUMBER to value 0"]
impl crate::Resettable for D_BATCH_NUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
