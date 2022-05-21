#[doc = "Register `D_DATAOUT_SIZE_1` reader"]
pub struct R(crate::R<D_DATAOUT_SIZE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAOUT_SIZE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAOUT_SIZE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAOUT_SIZE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAOUT_CHANNEL` reader - "]
pub struct DATAOUT_CHANNEL_R(crate::FieldReader<u16, u16>);
impl DATAOUT_CHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATAOUT_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAOUT_CHANNEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn dataout_channel(&self) -> DATAOUT_CHANNEL_R {
        DATAOUT_CHANNEL_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "Output cube’s channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dataout_size_1](index.html) module"]
pub struct D_DATAOUT_SIZE_1_SPEC;
impl crate::RegisterSpec for D_DATAOUT_SIZE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dataout_size_1::R](R) reader structure"]
impl crate::Readable for D_DATAOUT_SIZE_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATAOUT_SIZE_1 to value 0"]
impl crate::Resettable for D_DATAOUT_SIZE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
