#[doc = "Register `D_CONV_STRIDE_EXT` reader"]
pub struct R(crate::R<D_CONV_STRIDE_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CONV_STRIDE_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CONV_STRIDE_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CONV_STRIDE_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONV_X_STRIDE_EXT` reader - "]
pub struct CONV_X_STRIDE_EXT_R(crate::FieldReader<u8, u8>);
impl CONV_X_STRIDE_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONV_X_STRIDE_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONV_X_STRIDE_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONV_Y_STRIDE_EXT` reader - "]
pub struct CONV_Y_STRIDE_EXT_R(crate::FieldReader<u8, u8>);
impl CONV_Y_STRIDE_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONV_Y_STRIDE_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONV_Y_STRIDE_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn conv_x_stride_ext(&self) -> CONV_X_STRIDE_EXT_R {
        CONV_X_STRIDE_EXT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn conv_y_stride_ext(&self) -> CONV_Y_STRIDE_EXT_R {
        CONV_Y_STRIDE_EXT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Convolution x stride and convolution y stride after extension\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_conv_stride_ext](index.html) module"]
pub struct D_CONV_STRIDE_EXT_SPEC;
impl crate::RegisterSpec for D_CONV_STRIDE_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_conv_stride_ext::R](R) reader structure"]
impl crate::Readable for D_CONV_STRIDE_EXT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_CONV_STRIDE_EXT to value 0"]
impl crate::Resettable for D_CONV_STRIDE_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
