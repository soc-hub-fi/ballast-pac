#[doc = "Register `broadcast_bucket_depth` reader"]
pub struct R(crate::R<BROADCAST_BUCKET_DEPTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCAST_BUCKET_DEPTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCAST_BUCKET_DEPTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCAST_BUCKET_DEPTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `broadcast_bucket_depth` writer"]
pub struct W(crate::W<BROADCAST_BUCKET_DEPTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROADCAST_BUCKET_DEPTH_SPEC>;
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
impl From<crate::W<BROADCAST_BUCKET_DEPTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROADCAST_BUCKET_DEPTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `broadcast_bucket_depth` reader - broadcast_bucket_depth register is used to setting the bucket depth"]
pub type BROADCAST_BUCKET_DEPTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `broadcast_bucket_depth` writer - broadcast_bucket_depth register is used to setting the bucket depth"]
pub type BROADCAST_BUCKET_DEPTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BROADCAST_BUCKET_DEPTH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - broadcast_bucket_depth register is used to setting the bucket depth"]
    #[inline(always)]
    pub fn broadcast_bucket_depth(&self) -> BROADCAST_BUCKET_DEPTH_R {
        BROADCAST_BUCKET_DEPTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - broadcast_bucket_depth register is used to setting the bucket depth"]
    #[inline(always)]
    #[must_use]
    pub fn broadcast_bucket_depth(&mut self) -> BROADCAST_BUCKET_DEPTH_W<0> {
        BROADCAST_BUCKET_DEPTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "broadcast_bucket_depth register is used to setting the bucket depth\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcast_bucket_depth](index.html) module"]
pub struct BROADCAST_BUCKET_DEPTH_SPEC;
impl crate::RegisterSpec for BROADCAST_BUCKET_DEPTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcast_bucket_depth::R](R) reader structure"]
impl crate::Readable for BROADCAST_BUCKET_DEPTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [broadcast_bucket_depth::W](W) writer structure"]
impl crate::Writable for BROADCAST_BUCKET_DEPTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets broadcast_bucket_depth to value 0"]
impl crate::Resettable for BROADCAST_BUCKET_DEPTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
