#[doc = "Register `D_WEIGHT_SIZE_0` reader"]
pub struct R(crate::R<D_WEIGHT_SIZE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_SIZE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_SIZE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_SIZE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BYTE_PER_KERNEL` reader - "]
pub struct BYTE_PER_KERNEL_R(crate::FieldReader<u32>);
impl BYTE_PER_KERNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BYTE_PER_KERNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_PER_KERNEL_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn byte_per_kernel(&self) -> BYTE_PER_KERNEL_R {
        BYTE_PER_KERNEL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "The size of one kernel in bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_size_0](index.html) module"]
pub struct D_WEIGHT_SIZE_0_SPEC;
impl crate::RegisterSpec for D_WEIGHT_SIZE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_size_0::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_SIZE_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_WEIGHT_SIZE_0 to value 0"]
impl crate::Resettable for D_WEIGHT_SIZE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
