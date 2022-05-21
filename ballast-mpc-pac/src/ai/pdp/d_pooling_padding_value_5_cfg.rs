#[doc = "Register `D_POOLING_PADDING_VALUE_5_CFG` reader"]
pub struct R(crate::R<D_POOLING_PADDING_VALUE_5_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POOLING_PADDING_VALUE_5_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POOLING_PADDING_VALUE_5_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POOLING_PADDING_VALUE_5_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PAD_VALUE_5X` reader - "]
pub struct PAD_VALUE_5X_R(crate::FieldReader<u32, u32>);
impl PAD_VALUE_5X_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PAD_VALUE_5X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_VALUE_5X_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn pad_value_5x(&self) -> PAD_VALUE_5X_R {
        PAD_VALUE_5X_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
#[doc = "Padding_value*5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_padding_value_5_cfg](index.html) module"]
pub struct D_POOLING_PADDING_VALUE_5_CFG_SPEC;
impl crate::RegisterSpec for D_POOLING_PADDING_VALUE_5_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pooling_padding_value_5_cfg::R](R) reader structure"]
impl crate::Readable for D_POOLING_PADDING_VALUE_5_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_POOLING_PADDING_VALUE_5_CFG to value 0"]
impl crate::Resettable for D_POOLING_PADDING_VALUE_5_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
