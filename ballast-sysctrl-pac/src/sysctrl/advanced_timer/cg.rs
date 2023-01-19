#[doc = "Register `CG` reader"]
pub struct R(crate::R<CG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CG` writer"]
pub struct W(crate::W<CG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CG_SPEC>;
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
impl From<crate::W<CG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
pub type ENA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENA` writer - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
pub type ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<0> {
        ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMERS channels clock gating configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cg](index.html) module"]
pub struct CG_SPEC;
impl crate::RegisterSpec for CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cg::R](R) reader structure"]
impl crate::Readable for CG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cg::W](W) writer structure"]
impl crate::Writable for CG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CG to value 0"]
impl crate::Resettable for CG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
