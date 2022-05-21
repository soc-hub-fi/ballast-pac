#[doc = "Register `D_POST_Y_EXTENSION` reader"]
pub struct R(crate::R<D_POST_Y_EXTENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POST_Y_EXTENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POST_Y_EXTENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POST_Y_EXTENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Y_EXTENSION` reader - "]
pub struct Y_EXTENSION_R(crate::FieldReader<u8, u8>);
impl Y_EXTENSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Y_EXTENSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_EXTENSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn y_extension(&self) -> Y_EXTENSION_R {
        Y_EXTENSION_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Post extension parameter for image-in\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_post_y_extension](index.html) module"]
pub struct D_POST_Y_EXTENSION_SPEC;
impl crate::RegisterSpec for D_POST_Y_EXTENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_post_y_extension::R](R) reader structure"]
impl crate::Readable for D_POST_Y_EXTENSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_POST_Y_EXTENSION to value 0"]
impl crate::Resettable for D_POST_Y_EXTENSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
