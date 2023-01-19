#[doc = "Register `CDMA_IMAGE_IN_FORMATS_SEMI_0` reader"]
pub struct R(crate::R<CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CDMA_IMAGE_IN_FORMATS_SEMI` reader - "]
pub type CDMA_IMAGE_IN_FORMATS_SEMI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cdma_image_in_formats_semi(&self) -> CDMA_IMAGE_IN_FORMATS_SEMI_R {
        CDMA_IMAGE_IN_FORMATS_SEMI_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdma_image_in_formats_semi_0](index.html) module"]
pub struct CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC;
impl crate::RegisterSpec for CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdma_image_in_formats_semi_0::R](R) reader structure"]
impl crate::Readable for CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDMA_IMAGE_IN_FORMATS_SEMI_0 to value 0x03"]
impl crate::Resettable for CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
