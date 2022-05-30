#[doc = "Register `D_CLIP_CFG` reader"]
pub struct R(crate::R<D_CLIP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CLIP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CLIP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CLIP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLIP_TRUNCATE` reader - "]
pub struct CLIP_TRUNCATE_R(crate::FieldReader<u8>);
impl CLIP_TRUNCATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLIP_TRUNCATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLIP_TRUNCATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn clip_truncate(&self) -> CLIP_TRUNCATE_R {
        CLIP_TRUNCATE_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Number of bits to be truncated before sending to SDP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_clip_cfg](index.html) module"]
pub struct D_CLIP_CFG_SPEC;
impl crate::RegisterSpec for D_CLIP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_clip_cfg::R](R) reader structure"]
impl crate::Readable for D_CLIP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_CLIP_CFG to value 0"]
impl crate::Resettable for D_CLIP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
