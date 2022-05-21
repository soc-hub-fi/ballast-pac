#[doc = "Register `D_PIXEL_OFFSET` reader"]
pub struct R(crate::R<D_PIXEL_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PIXEL_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PIXEL_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PIXEL_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIXEL_X_OFFSET` reader - "]
pub struct PIXEL_X_OFFSET_R(crate::FieldReader<u8, u8>);
impl PIXEL_X_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIXEL_X_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIXEL_X_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIXEL_Y_OFFSET` reader - "]
pub struct PIXEL_Y_OFFSET_R(crate::FieldReader<u8, u8>);
impl PIXEL_Y_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIXEL_Y_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIXEL_Y_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pixel_x_offset(&self) -> PIXEL_X_OFFSET_R {
        PIXEL_X_OFFSET_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pixel_y_offset(&self) -> PIXEL_Y_OFFSET_R {
        PIXEL_Y_OFFSET_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "For image-in mode, horizontal offset and vertical offset of the 1 st pixel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pixel_offset](index.html) module"]
pub struct D_PIXEL_OFFSET_SPEC;
impl crate::RegisterSpec for D_PIXEL_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pixel_offset::R](R) reader structure"]
impl crate::Readable for D_PIXEL_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PIXEL_OFFSET to value 0"]
impl crate::Resettable for D_PIXEL_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
