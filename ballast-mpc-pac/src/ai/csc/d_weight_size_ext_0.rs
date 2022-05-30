#[doc = "Register `D_WEIGHT_SIZE_EXT_0` reader"]
pub struct R(crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WEIGHT_WIDTH_EXT` reader - "]
pub struct WEIGHT_WIDTH_EXT_R(crate::FieldReader<u8>);
impl WEIGHT_WIDTH_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WEIGHT_WIDTH_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_WIDTH_EXT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_HEIGHT_EXT` reader - "]
pub struct WEIGHT_HEIGHT_EXT_R(crate::FieldReader<u8>);
impl WEIGHT_HEIGHT_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WEIGHT_HEIGHT_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_HEIGHT_EXT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn weight_width_ext(&self) -> WEIGHT_WIDTH_EXT_R {
        WEIGHT_WIDTH_EXT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_height_ext(&self) -> WEIGHT_HEIGHT_EXT_R {
        WEIGHT_HEIGHT_EXT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Weight’s width and height after extension\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_size_ext_0](index.html) module"]
pub struct D_WEIGHT_SIZE_EXT_0_SPEC;
impl crate::RegisterSpec for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_size_ext_0::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_WEIGHT_SIZE_EXT_0 to value 0"]
impl crate::Resettable for D_WEIGHT_SIZE_EXT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
