#[doc = "Register `PADCFG_08_15` reader"]
pub struct R(crate::R<PADCFG_08_15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG_08_15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG_08_15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG_08_15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG_08_15` writer"]
pub struct W(crate::W<PADCFG_08_15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG_08_15_SPEC>;
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
impl From<crate::W<PADCFG_08_15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG_08_15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG_08_15` reader - "]
pub struct PADCFG_08_15_R(crate::FieldReader<u32>);
impl PADCFG_08_15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADCFG_08_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADCFG_08_15_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADCFG_08_15` writer - "]
pub struct PADCFG_08_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PADCFG_08_15_W<'a> {
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
    pub fn padcfg_08_15(&self) -> PADCFG_08_15_R {
        PADCFG_08_15_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg_08_15(&mut self) -> PADCFG_08_15_W {
        PADCFG_08_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg_08_15](index.html) module"]
pub struct PADCFG_08_15_SPEC;
impl crate::RegisterSpec for PADCFG_08_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg_08_15::R](R) reader structure"]
impl crate::Readable for PADCFG_08_15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg_08_15::W](W) writer structure"]
impl crate::Writable for PADCFG_08_15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADCFG_08_15 to value 0"]
impl crate::Resettable for PADCFG_08_15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
