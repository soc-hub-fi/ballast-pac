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
pub struct BROADCAST_BUCKET_INTERVAL_R(crate::FieldReader<u16>);
impl BROADCAST_BUCKET_INTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BROADCAST_BUCKET_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROADCAST_BUCKET_INTERVAL_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `broadcast_bucket_interval` writer - The bucket wil be periodically refilled after broadcast_bucket_interval time"]
pub struct BROADCAST_BUCKET_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BROADCAST_BUCKET_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn broadcast_bucket_interval(&mut self) -> BROADCAST_BUCKET_INTERVAL_W {
        BROADCAST_BUCKET_INTERVAL_W { w: self }
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
}
#[doc = "`reset()` method sets broadcast_bucket_interval to value 0"]
impl crate::Resettable for BROADCAST_BUCKET_INTERVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
