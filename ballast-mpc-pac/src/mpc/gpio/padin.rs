#[doc = "Register `PADIN` reader"]
pub struct R(crate::R<PADIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADIN` writer"]
pub struct W(crate::W<PADIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADIN_SPEC>;
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
impl From<crate::W<PADIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADIN` reader - "]
pub struct PADIN_R(crate::FieldReader<u32, u32>);
impl PADIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADIN` writer - "]
pub struct PADIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padin(&self) -> PADIN_R {
        PADIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padin(&mut self) -> PADIN_W {
        PADIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padin](index.html) module"]
pub struct PADIN_SPEC;
impl crate::RegisterSpec for PADIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padin::R](R) reader structure"]
impl crate::Readable for PADIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padin::W](W) writer structure"]
impl crate::Writable for PADIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADIN to value 0"]
impl crate::Resettable for PADIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
