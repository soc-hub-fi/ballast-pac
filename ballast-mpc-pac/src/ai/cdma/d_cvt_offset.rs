#[doc = "Register `D_CVT_OFFSET` reader"]
pub struct R(crate::R<D_CVT_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVT_OFFSET` reader - "]
pub struct CVT_OFFSET_R(crate::FieldReader<u16>);
impl CVT_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CVT_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVT_OFFSET_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvt_offset(&self) -> CVT_OFFSET_R {
        CVT_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Offset of input data convertor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_offset](index.html) module"]
pub struct D_CVT_OFFSET_SPEC;
impl crate::RegisterSpec for D_CVT_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_offset::R](R) reader structure"]
impl crate::Readable for D_CVT_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_CVT_OFFSET to value 0"]
impl crate::Resettable for D_CVT_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
