#[doc = "Register `CMP_HI` reader"]
pub struct R(crate::R<CMP_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP_HI` writer"]
pub struct W(crate::W<CMP_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP_HI_SPEC>;
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
impl From<crate::W<CMP_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_HI` reader - "]
pub struct CMP_HI_R(crate::FieldReader<u32, u32>);
impl CMP_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMP_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_HI` writer - "]
pub struct CMP_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_HI_W<'a> {
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
    pub fn cmp_hi(&self) -> CMP_HI_R {
        CMP_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp_hi(&mut self) -> CMP_HI_W {
        CMP_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer High comparator value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_hi](index.html) module"]
pub struct CMP_HI_SPEC;
impl crate::RegisterSpec for CMP_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp_hi::R](R) reader structure"]
impl crate::Readable for CMP_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp_hi::W](W) writer structure"]
impl crate::Writable for CMP_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP_HI to value 0"]
impl crate::Resettable for CMP_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
