#[doc = "Register `D_CVT_SHIFT` reader"]
pub struct R(crate::R<D_CVT_SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVT_SHIFT` reader - "]
pub struct CVT_SHIFT_R(crate::FieldReader<u8>);
impl CVT_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CVT_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVT_SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cvt_shift(&self) -> CVT_SHIFT_R {
        CVT_SHIFT_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Output converter shifter value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_shift](index.html) module"]
pub struct D_CVT_SHIFT_SPEC;
impl crate::RegisterSpec for D_CVT_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_shift::R](R) reader structure"]
impl crate::Readable for D_CVT_SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_CVT_SHIFT to value 0"]
impl crate::Resettable for D_CVT_SHIFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
