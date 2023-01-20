#[doc = "Register `broadcast_bucket_interval` reader"]
pub struct R(crate::R<BROADCAST_BUCKET_INTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCAST_BUCKET_INTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCAST_BUCKET_INTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCAST_BUCKET_INTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `broadcast_bucket_interval` writer"]
pub struct W(crate::W<BROADCAST_BUCKET_INTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROADCAST_BUCKET_INTERVAL_SPEC>;
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
impl From<crate::W<BROADCAST_BUCKET_INTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROADCAST_BUCKET_INTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `broadcast_bucket_interval` reader - The bucket wil be periodically refilled after broadcast_bucket_interval time"]
pub type BROADCAST_BUCKET_INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `broadcast_bucket_interval` writer - The bucket wil be periodically refilled after broadcast_bucket_interval time"]
pub type BROADCAST_BUCKET_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BROADCAST_BUCKET_INTERVAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The bucket wil be periodically refilled after broadcast_bucket_interval time"]
    #[inline(always)]
    pub fn broadcast_bucket_interval(&self) -> BROADCAST_BUCKET_INTERVAL_R {
        BROADCAST_BUCKET_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bucket wil be periodically refilled after broadcast_bucket_interval time"]
    #[inline(always)]
    #[must_use]
    pub fn broadcast_bucket_interval(&mut self) -> BROADCAST_BUCKET_INTERVAL_W<0> {
        BROADCAST_BUCKET_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The bucket wil be periodically refilled after broadcast_bucket_interval time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcast_bucket_interval](index.html) module"]
pub struct BROADCAST_BUCKET_INTERVAL_SPEC;
impl crate::RegisterSpec for BROADCAST_BUCKET_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcast_bucket_interval::R](R) reader structure"]
impl crate::Readable for BROADCAST_BUCKET_INTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [broadcast_bucket_interval::W](W) writer structure"]
impl crate::Writable for BROADCAST_BUCKET_INTERVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets broadcast_bucket_interval to value 0"]
impl crate::Resettable for BROADCAST_BUCKET_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
