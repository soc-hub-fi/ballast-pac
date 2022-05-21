#[doc = "Register `D_CVT_SHIFT` reader"]
pub struct R(crate::R<D_CVT_SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_CVT_SHIFT` writer"]
pub struct W(crate::W<D_CVT_SHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CVT_SHIFT_SPEC>;
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
impl From<crate::W<D_CVT_SHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CVT_SHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVT_SHIFT` reader - "]
pub struct CVT_SHIFT_R(crate::FieldReader<u8, u8>);
impl CVT_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CVT_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVT_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVT_SHIFT` writer - "]
pub struct CVT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> CVT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cvt_shift(&self) -> CVT_SHIFT_R {
        CVT_SHIFT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cvt_shift(&mut self) -> CVT_SHIFT_W {
        CVT_SHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output converter shifter value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_shift](index.html) module"]
pub struct D_CVT_SHIFT_SPEC;
impl crate::RegisterSpec for D_CVT_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_shift::R](R) reader structure"]
impl crate::Readable for D_CVT_SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_cvt_shift::W](W) writer structure"]
impl crate::Writable for D_CVT_SHIFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_CVT_SHIFT to value 0"]
impl crate::Resettable for D_CVT_SHIFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
