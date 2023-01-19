#[doc = "Register `D_CVT_SCALE` reader"]
pub struct R(crate::R<D_CVT_SCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_SCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_SCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_SCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_CVT_SCALE` writer"]
pub struct W(crate::W<D_CVT_SCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CVT_SCALE_SPEC>;
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
impl From<crate::W<D_CVT_SCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CVT_SCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVT_SCALE` reader - "]
pub type CVT_SCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CVT_SCALE` writer - "]
pub type CVT_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_CVT_SCALE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvt_scale(&self) -> CVT_SCALE_R {
        CVT_SCALE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cvt_scale(&mut self) -> CVT_SCALE_W<0> {
        CVT_SCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output converter scale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_scale](index.html) module"]
pub struct D_CVT_SCALE_SPEC;
impl crate::RegisterSpec for D_CVT_SCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_scale::R](R) reader structure"]
impl crate::Readable for D_CVT_SCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_cvt_scale::W](W) writer structure"]
impl crate::Writable for D_CVT_SCALE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_CVT_SCALE to value 0"]
impl crate::Resettable for D_CVT_SCALE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
