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
#[doc = "Field `PAD_LEFT` reader - "]
pub struct PAD_LEFT_R(crate::FieldReader<u8, u8>);
impl PAD_LEFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_LEFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_LEFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_TOP` reader - "]
pub struct PAD_TOP_R(crate::FieldReader<u8, u8>);
impl PAD_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_TOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_RIGHT` reader - "]
pub struct PAD_RIGHT_R(crate::FieldReader<u8, u8>);
impl PAD_RIGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_RIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_RIGHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_BOTTOM` reader - "]
pub struct PAD_BOTTOM_R(crate::FieldReader<u8, u8>);
impl PAD_BOTTOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_BOTTOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_BOTTOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pad_left(&self) -> PAD_LEFT_R {
        PAD_LEFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pad_top(&self) -> PAD_TOP_R {
        PAD_TOP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pad_right(&self) -> PAD_RIGHT_R {
        PAD_RIGHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pad_bottom(&self) -> PAD_BOTTOM_R {
        PAD_BOTTOM_R::new(((self.bits >> 12) & 7) as u8)
    }
}
#[doc = "Left/right/top/bottom padding size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_padding_cfg](index.html) module"]
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
