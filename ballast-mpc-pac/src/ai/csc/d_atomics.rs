#[doc = "Register `D_ATOMICS` reader"]
pub struct R(crate::R<D_ATOMICS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ATOMICS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ATOMICS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ATOMICS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ATOMICS` reader - "]
pub struct ATOMICS_R(crate::FieldReader<u32>);
impl ATOMICS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ATOMICS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOMICS_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:20"]
    #[inline(always)]
    pub fn atomics(&self) -> ATOMICS_R {
        ATOMICS_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
#[doc = "Equals to output_data_cube_width * output_data_cube_height - 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_atomics](index.html) module"]
pub struct D_ATOMICS_SPEC;
impl crate::RegisterSpec for D_ATOMICS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_atomics::R](R) reader structure"]
impl crate::Readable for D_ATOMICS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_ATOMICS to value 0x01"]
impl crate::Resettable for D_ATOMICS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
