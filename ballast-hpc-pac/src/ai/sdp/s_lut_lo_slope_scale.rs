#[doc = "Register `S_LUT_LO_SLOPE_SCALE` reader"]
pub struct R(crate::R<S_LUT_LO_SLOPE_SCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LO_SLOPE_SCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LO_SLOPE_SCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LO_SLOPE_SCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_LO_SLOPE_SCALE` writer"]
pub struct W(crate::W<S_LUT_LO_SLOPE_SCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_LO_SLOPE_SCALE_SPEC>;
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
impl From<crate::W<S_LUT_LO_SLOPE_SCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_LO_SLOPE_SCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SCALE` reader - "]
pub type LUT_LO_SLOPE_UFLOW_SCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SCALE` writer - "]
pub type LUT_LO_SLOPE_UFLOW_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_LO_SLOPE_SCALE_SPEC, u16, u16, 16, O>;
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SCALE` reader - "]
pub type LUT_LO_SLOPE_OFLOW_SCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SCALE` writer - "]
pub type LUT_LO_SLOPE_OFLOW_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_LO_SLOPE_SCALE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_lo_slope_uflow_scale(&self) -> LUT_LO_SLOPE_UFLOW_SCALE_R {
        LUT_LO_SLOPE_UFLOW_SCALE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lut_lo_slope_oflow_scale(&self) -> LUT_LO_SLOPE_OFLOW_SCALE_R {
        LUT_LO_SLOPE_OFLOW_SCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn lut_lo_slope_uflow_scale(&mut self) -> LUT_LO_SLOPE_UFLOW_SCALE_W<0> {
        LUT_LO_SLOPE_UFLOW_SCALE_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn lut_lo_slope_oflow_scale(&mut self) -> LUT_LO_SLOPE_OFLOW_SCALE_W<16> {
        LUT_LO_SLOPE_OFLOW_SCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope scale parameter for LO LUT underflow and overflow, signed value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_lo_slope_scale](index.html) module"]
pub struct S_LUT_LO_SLOPE_SCALE_SPEC;
impl crate::RegisterSpec for S_LUT_LO_SLOPE_SCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_lo_slope_scale::R](R) reader structure"]
impl crate::Readable for S_LUT_LO_SLOPE_SCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_lo_slope_scale::W](W) writer structure"]
impl crate::Writable for S_LUT_LO_SLOPE_SCALE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_LO_SLOPE_SCALE to value 0"]
impl crate::Resettable for S_LUT_LO_SLOPE_SCALE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
