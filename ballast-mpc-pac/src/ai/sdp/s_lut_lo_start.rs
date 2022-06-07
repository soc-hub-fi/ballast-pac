#[doc = "Register `S_LUT_LO_START` reader"]
pub struct R(crate::R<S_LUT_LO_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LO_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LO_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LO_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_LO_START` writer"]
pub struct W(crate::W<S_LUT_LO_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_LO_START_SPEC>;
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
impl From<crate::W<S_LUT_LO_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_LO_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LO_START` reader - "]
pub struct LUT_LO_START_R(crate::FieldReader<u32>);
impl LUT_LO_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LUT_LO_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LO_START_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LO_START` writer - "]
pub struct LUT_LO_START_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LO_START_W<'a> {
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
    pub fn lut_lo_start(&self) -> LUT_LO_START_R {
        LUT_LO_START_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lut_lo_start(&mut self) -> LUT_LO_START_W {
        LUT_LO_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start of LO LUT’s range\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_lo_start](index.html) module"]
pub struct S_LUT_LO_START_SPEC;
impl crate::RegisterSpec for S_LUT_LO_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_lo_start::R](R) reader structure"]
impl crate::Readable for S_LUT_LO_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_lo_start::W](W) writer structure"]
impl crate::Writable for S_LUT_LO_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_LUT_LO_START to value 0"]
impl crate::Resettable for S_LUT_LO_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
