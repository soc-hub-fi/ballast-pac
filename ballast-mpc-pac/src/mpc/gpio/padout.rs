#[doc = "Register `PADOUT` reader"]
pub struct R(crate::R<PADOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUT` writer"]
pub struct W(crate::W<PADOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUT_SPEC>;
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
impl From<crate::W<PADOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADOUT` reader - "]
pub struct PADOUT_R(crate::FieldReader<u32, u32>);
impl PADOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADOUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADOUT` writer - "]
pub struct PADOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PADOUT_W<'a> {
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
    pub fn padout(&self) -> PADOUT_R {
        PADOUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padout(&mut self) -> PADOUT_W {
        PADOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padout](index.html) module"]
pub struct PADOUT_SPEC;
impl crate::RegisterSpec for PADOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padout::R](R) reader structure"]
impl crate::Readable for PADOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padout::W](W) writer structure"]
impl crate::Writable for PADOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADOUT to value 0"]
impl crate::Resettable for PADOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
