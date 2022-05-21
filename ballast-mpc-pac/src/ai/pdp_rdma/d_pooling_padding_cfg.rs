#[doc = "Register `D_POOLING_PADDING_CFG` reader"]
pub struct R(crate::R<D_POOLING_PADDING_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POOLING_PADDING_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POOLING_PADDING_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POOLING_PADDING_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PAD_WIDTH` reader - "]
pub struct PAD_WIDTH_R(crate::FieldReader<u8, u8>);
impl PAD_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pad_width(&self) -> PAD_WIDTH_R {
        PAD_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_padding_cfg](index.html) module"]
pub struct D_POOLING_PADDING_CFG_SPEC;
impl crate::RegisterSpec for D_POOLING_PADDING_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pooling_padding_cfg::R](R) reader structure"]
impl crate::Readable for D_POOLING_PADDING_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_POOLING_PADDING_CFG to value 0"]
impl crate::Resettable for D_POOLING_PADDING_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
