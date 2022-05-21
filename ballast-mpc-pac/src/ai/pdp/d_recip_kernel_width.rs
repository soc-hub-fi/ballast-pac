#[doc = "Register `D_RECIP_KERNEL_WIDTH` reader"]
pub struct R(crate::R<D_RECIP_KERNEL_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_RECIP_KERNEL_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_RECIP_KERNEL_WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_RECIP_KERNEL_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECIP_KERNEL_WIDTH` reader - "]
pub struct RECIP_KERNEL_WIDTH_R(crate::FieldReader<u32, u32>);
impl RECIP_KERNEL_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RECIP_KERNEL_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECIP_KERNEL_WIDTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn recip_kernel_width(&self) -> RECIP_KERNEL_WIDTH_R {
        RECIP_KERNEL_WIDTH_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
#[doc = "Reciprocal of pooling kernel width, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_recip_kernel_width](index.html) module"]
pub struct D_RECIP_KERNEL_WIDTH_SPEC;
impl crate::RegisterSpec for D_RECIP_KERNEL_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_recip_kernel_width::R](R) reader structure"]
impl crate::Readable for D_RECIP_KERNEL_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_RECIP_KERNEL_WIDTH to value 0"]
impl crate::Resettable for D_RECIP_KERNEL_WIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
