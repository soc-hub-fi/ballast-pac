#[doc = "Register `PADCFG_32_39` reader"]
pub struct R(crate::R<PADCFG_32_39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG_32_39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG_32_39_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG_32_39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG_32_39` writer"]
pub struct W(crate::W<PADCFG_32_39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG_32_39_SPEC>;
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
impl From<crate::W<PADCFG_32_39_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG_32_39_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG_32_39` reader - "]
pub type PADCFG_32_39_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PADCFG_32_39` writer - "]
pub type PADCFG_32_39_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCFG_32_39_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg_32_39(&self) -> PADCFG_32_39_R {
        PADCFG_32_39_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_32_39(&mut self) -> PADCFG_32_39_W<0> {
        PADCFG_32_39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg_32_39](index.html) module"]
pub struct PADCFG_32_39_SPEC;
impl crate::RegisterSpec for PADCFG_32_39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg_32_39::R](R) reader structure"]
impl crate::Readable for PADCFG_32_39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg_32_39::W](W) writer structure"]
impl crate::Writable for PADCFG_32_39_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCFG_32_39 to value 0"]
impl crate::Resettable for PADCFG_32_39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
