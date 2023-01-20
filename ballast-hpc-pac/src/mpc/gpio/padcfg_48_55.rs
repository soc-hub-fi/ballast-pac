#[doc = "Register `PADCFG_48_55` reader"]
pub struct R(crate::R<PADCFG_48_55_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG_48_55_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG_48_55_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG_48_55_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG_48_55` writer"]
pub struct W(crate::W<PADCFG_48_55_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG_48_55_SPEC>;
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
impl From<crate::W<PADCFG_48_55_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG_48_55_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG_48_55` reader - "]
pub type PADCFG_48_55_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PADCFG_48_55` writer - "]
pub type PADCFG_48_55_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCFG_48_55_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg_48_55(&self) -> PADCFG_48_55_R {
        PADCFG_48_55_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_48_55(&mut self) -> PADCFG_48_55_W<0> {
        PADCFG_48_55_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg_48_55](index.html) module"]
pub struct PADCFG_48_55_SPEC;
impl crate::RegisterSpec for PADCFG_48_55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg_48_55::R](R) reader structure"]
impl crate::Readable for PADCFG_48_55_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg_48_55::W](W) writer structure"]
impl crate::Writable for PADCFG_48_55_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCFG_48_55 to value 0"]
impl crate::Resettable for PADCFG_48_55_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
