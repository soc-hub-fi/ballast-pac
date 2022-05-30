#[doc = "Register `GPIOEN` reader"]
pub struct R(crate::R<GPIOEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOEN` writer"]
pub struct W(crate::W<GPIOEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOEN_SPEC>;
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
impl From<crate::W<GPIOEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOEN` reader - "]
pub struct GPIOEN_R(crate::FieldReader<u32>);
impl GPIOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOEN_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOEN` writer - "]
pub struct GPIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEN_W<'a> {
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
    pub fn gpioen(&self) -> GPIOEN_R {
        GPIOEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpioen(&mut self) -> GPIOEN_W {
        GPIOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the enable bit per GPIO line.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioen](index.html) module"]
pub struct GPIOEN_SPEC;
impl crate::RegisterSpec for GPIOEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioen::R](R) reader structure"]
impl crate::Readable for GPIOEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioen::W](W) writer structure"]
impl crate::Writable for GPIOEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOEN to value 0"]
impl crate::Resettable for GPIOEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
