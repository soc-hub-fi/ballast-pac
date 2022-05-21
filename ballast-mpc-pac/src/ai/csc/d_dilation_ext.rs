#[doc = "Register `D_DILATION_EXT` reader"]
pub struct R(crate::R<D_DILATION_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DILATION_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DILATION_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DILATION_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `X_DILATION_EXT` reader - "]
pub struct X_DILATION_EXT_R(crate::FieldReader<u8, u8>);
impl X_DILATION_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X_DILATION_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X_DILATION_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Y_DILATION_EXT` reader - "]
pub struct Y_DILATION_EXT_R(crate::FieldReader<u8, u8>);
impl Y_DILATION_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Y_DILATION_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_DILATION_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn x_dilation_ext(&self) -> X_DILATION_EXT_R {
        X_DILATION_EXT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn y_dilation_ext(&self) -> Y_DILATION_EXT_R {
        Y_DILATION_EXT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Dilation parameter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dilation_ext](index.html) module"]
pub struct D_DILATION_EXT_SPEC;
impl crate::RegisterSpec for D_DILATION_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dilation_ext::R](R) reader structure"]
impl crate::Readable for D_DILATION_EXT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DILATION_EXT to value 0"]
impl crate::Resettable for D_DILATION_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
