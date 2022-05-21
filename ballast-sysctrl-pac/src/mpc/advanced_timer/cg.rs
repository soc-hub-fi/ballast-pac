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
pub struct ENA_R(crate::FieldReader<u16, u16>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
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
}
#[doc = "`reset()` method sets CG to value 0"]
impl crate::Resettable for CG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
