#[doc = "Register `BINCU_TH` reader"]
pub struct R(crate::R<BINCU_TH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINCU_TH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINCU_TH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINCU_TH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BINCU_TH` writer"]
pub struct W(crate::W<BINCU_TH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BINCU_TH_SPEC>;
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
impl From<crate::W<BINCU_TH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BINCU_TH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINCU_TH` reader - "]
pub struct BINCU_TH_R(crate::FieldReader<u32>);
impl BINCU_TH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BINCU_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BINCU_TH_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BINCU_TH` writer - "]
pub struct BINCU_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> BINCU_TH_W<'a> {
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
    pub fn bincu_th(&self) -> BINCU_TH_R {
        BINCU_TH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bincu_th(&mut self) -> BINCU_TH_W {
        BINCU_TH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER binarization threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bincu_th](index.html) module"]
pub struct BINCU_TH_SPEC;
impl crate::RegisterSpec for BINCU_TH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bincu_th::R](R) reader structure"]
impl crate::Readable for BINCU_TH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bincu_th::W](W) writer structure"]
impl crate::Writable for BINCU_TH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BINCU_TH to value 0"]
impl crate::Resettable for BINCU_TH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
