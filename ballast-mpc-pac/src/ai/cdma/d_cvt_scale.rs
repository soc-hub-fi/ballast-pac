#[doc = "Register `D_CVT_SCALE` reader"]
pub struct R(crate::R<D_CVT_SCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_SCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_SCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_SCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVT_SCALE` reader - "]
pub struct CVT_SCALE_R(crate::FieldReader<u16>);
impl CVT_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CVT_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVT_SCALE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvt_scale(&self) -> CVT_SCALE_R {
        CVT_SCALE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Scale of input data convertor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_scale](index.html) module"]
pub struct D_CVT_SCALE_SPEC;
impl crate::RegisterSpec for D_CVT_SCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_scale::R](R) reader structure"]
impl crate::Readable for D_CVT_SCALE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_CVT_SCALE to value 0"]
impl crate::Resettable for D_CVT_SCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
