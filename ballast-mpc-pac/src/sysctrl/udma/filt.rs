#[doc = "Register `FILT` reader"]
pub struct R(crate::R<FILT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILT` writer"]
pub struct W(crate::W<FILT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILT_SPEC>;
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
impl From<crate::W<FILT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT` reader - "]
pub type FILT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FILT` writer - "]
pub type FILT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<0> {
        FILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER control mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt](index.html) module"]
pub struct FILT_SPEC;
impl crate::RegisterSpec for FILT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filt::R](R) reader structure"]
impl crate::Readable for FILT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filt::W](W) writer structure"]
impl crate::Writable for FILT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILT to value 0"]
impl crate::Resettable for FILT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
