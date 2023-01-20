#[doc = "Register `PADOUTSET_00_31` reader"]
pub struct R(crate::R<PADOUTSET_00_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUTSET_00_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUTSET_00_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUTSET_00_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUTSET_00_31` writer"]
pub struct W(crate::W<PADOUTSET_00_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUTSET_00_31_SPEC>;
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
impl From<crate::W<PADOUTSET_00_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUTSET_00_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADOUTSET` reader - "]
pub type PADOUTSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PADOUTSET` writer - "]
pub type PADOUTSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADOUTSET_00_31_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padoutset(&self) -> PADOUTSET_R {
        PADOUTSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padoutset(&mut self) -> PADOUTSET_W<0> {
        PADOUTSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO pad output set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padoutset_00_31](index.html) module"]
pub struct PADOUTSET_00_31_SPEC;
impl crate::RegisterSpec for PADOUTSET_00_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padoutset_00_31::R](R) reader structure"]
impl crate::Readable for PADOUTSET_00_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padoutset_00_31::W](W) writer structure"]
impl crate::Writable for PADOUTSET_00_31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADOUTSET_00_31 to value 0"]
impl crate::Resettable for PADOUTSET_00_31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
