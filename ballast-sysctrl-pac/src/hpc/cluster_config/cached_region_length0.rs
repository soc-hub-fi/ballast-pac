#[doc = "Register `cached_region_length0` reader"]
pub struct R(crate::R<CACHED_REGION_LENGTH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHED_REGION_LENGTH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHED_REGION_LENGTH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHED_REGION_LENGTH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cached_region_length0` writer"]
pub struct W(crate::W<CACHED_REGION_LENGTH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHED_REGION_LENGTH0_SPEC>;
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
impl From<crate::W<CACHED_REGION_LENGTH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHED_REGION_LENGTH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cached_region_length0` reader - "]
pub struct CACHED_REGION_LENGTH0_R(crate::FieldReader<u64, u64>);
impl CACHED_REGION_LENGTH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        CACHED_REGION_LENGTH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHED_REGION_LENGTH0_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cached_region_length0` writer - "]
pub struct CACHED_REGION_LENGTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHED_REGION_LENGTH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn cached_region_length0(&self) -> CACHED_REGION_LENGTH0_R {
        CACHED_REGION_LENGTH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn cached_region_length0(&mut self) -> CACHED_REGION_LENGTH0_W {
        CACHED_REGION_LENGTH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cached_region_length0](index.html) module"]
pub struct CACHED_REGION_LENGTH0_SPEC;
impl crate::RegisterSpec for CACHED_REGION_LENGTH0_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [cached_region_length0::R](R) reader structure"]
impl crate::Readable for CACHED_REGION_LENGTH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cached_region_length0::W](W) writer structure"]
impl crate::Writable for CACHED_REGION_LENGTH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cached_region_length0 to value 0"]
impl crate::Resettable for CACHED_REGION_LENGTH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
