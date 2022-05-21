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
#[doc = "Register `D_CLIP_CFG` writer"]
pub struct W(crate::W<D_CLIP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CLIP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<D_CLIP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CLIP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIP_TRUNCATE` reader - "]
pub struct CLIP_TRUNCATE_R(crate::FieldReader<u8, u8>);
impl CLIP_TRUNCATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLIP_TRUNCATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLIP_TRUNCATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLIP_TRUNCATE` writer - "]
pub struct CLIP_TRUNCATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIP_TRUNCATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn clip_truncate(&self) -> CLIP_TRUNCATE_R {
        CLIP_TRUNCATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn clip_truncate(&mut self) -> CLIP_TRUNCATE_W {
        CLIP_TRUNCATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of bits to be truncated before sending to SDP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_clip_cfg](index.html) module"]
pub struct D_CLIP_CFG_SPEC;
impl crate::RegisterSpec for D_CLIP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_clip_cfg::R](R) reader structure"]
impl crate::Readable for D_CLIP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_clip_cfg::W](W) writer structure"]
impl crate::Writable for D_CLIP_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_CLIP_CFG to value 0"]
impl crate::Resettable for D_CLIP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
