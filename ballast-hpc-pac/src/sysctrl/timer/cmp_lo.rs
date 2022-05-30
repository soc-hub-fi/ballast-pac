#[doc = "Register `CMP_LO` reader"]
pub struct R(crate::R<CMP_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP_LO` writer"]
pub struct W(crate::W<CMP_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP_LO_SPEC>;
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
impl From<crate::W<CMP_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_LO` reader - "]
pub struct CMP_LO_R(crate::FieldReader<u32>);
impl CMP_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMP_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_LO_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_LO` writer - "]
pub struct CMP_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_LO_W<'a> {
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
    pub fn cmp_lo(&self) -> CMP_LO_R {
        CMP_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp_lo(&mut self) -> CMP_LO_W {
        CMP_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Low comparator value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_lo](index.html) module"]
pub struct CMP_LO_SPEC;
impl crate::RegisterSpec for CMP_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp_lo::R](R) reader structure"]
impl crate::Readable for CMP_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp_lo::W](W) writer structure"]
impl crate::Writable for CMP_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP_LO to value 0"]
impl crate::Resettable for CMP_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
