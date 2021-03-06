#[doc = "Register `mtimecmp_0` reader"]
pub struct R(crate::R<MTIMECMP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIMECMP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIMECMP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIMECMP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mtimecmp_0` writer"]
pub struct W(crate::W<MTIMECMP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIMECMP_0_SPEC>;
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
impl From<crate::W<MTIMECMP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIMECMP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mtimecmp` reader - "]
pub struct MTIMECMP_R(crate::FieldReader<u64>);
impl MTIMECMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        MTIMECMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIMECMP_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mtimecmp` writer - "]
pub struct MTIMECMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MTIMECMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn mtimecmp(&self) -> MTIMECMP_R {
        MTIMECMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn mtimecmp(&mut self) -> MTIMECMP_W {
        MTIMECMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Machine mode timer compare register for Hart 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtimecmp_0](index.html) module"]
pub struct MTIMECMP_0_SPEC;
impl crate::RegisterSpec for MTIMECMP_0_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtimecmp_0::R](R) reader structure"]
impl crate::Readable for MTIMECMP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtimecmp_0::W](W) writer structure"]
impl crate::Writable for MTIMECMP_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mtimecmp_0 to value 0"]
impl crate::Resettable for MTIMECMP_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
