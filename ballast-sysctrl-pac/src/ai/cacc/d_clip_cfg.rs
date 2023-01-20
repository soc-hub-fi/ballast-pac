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
pub type CLIP_TRUNCATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLIP_TRUNCATE` writer - "]
pub type CLIP_TRUNCATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_CLIP_CFG_SPEC, u8, u8, 5, O>;
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
    #[must_use]
    pub fn clip_truncate(&mut self) -> CLIP_TRUNCATE_W<0> {
        CLIP_TRUNCATE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_CLIP_CFG to value 0"]
impl crate::Resettable for D_CLIP_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
