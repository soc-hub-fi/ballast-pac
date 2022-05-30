#[doc = "Register `D_DATA_CUBE_OUT_WIDTH` reader"]
pub struct R(crate::R<D_DATA_CUBE_OUT_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATA_CUBE_OUT_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATA_CUBE_OUT_WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATA_CUBE_OUT_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUBE_OUT_WIDTH` reader - "]
pub struct CUBE_OUT_WIDTH_R(crate::FieldReader<u16>);
impl CUBE_OUT_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CUBE_OUT_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUBE_OUT_WIDTH_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn cube_out_width(&self) -> CUBE_OUT_WIDTH_R {
        CUBE_OUT_WIDTH_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "Output data cube’s width\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_data_cube_out_width](index.html) module"]
pub struct D_DATA_CUBE_OUT_WIDTH_SPEC;
impl crate::RegisterSpec for D_DATA_CUBE_OUT_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_data_cube_out_width::R](R) reader structure"]
impl crate::Readable for D_DATA_CUBE_OUT_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATA_CUBE_OUT_WIDTH to value 0"]
impl crate::Resettable for D_DATA_CUBE_OUT_WIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
