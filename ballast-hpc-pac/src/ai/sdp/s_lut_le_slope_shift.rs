#[doc = "Register `S_LUT_LE_SLOPE_SHIFT` reader"]
pub struct R(crate::R<S_LUT_LE_SLOPE_SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LE_SLOPE_SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LE_SLOPE_SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LE_SLOPE_SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_LE_SLOPE_SHIFT` writer"]
pub struct W(crate::W<S_LUT_LE_SLOPE_SHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_LE_SLOPE_SHIFT_SPEC>;
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
impl From<crate::W<S_LUT_LE_SLOPE_SHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_LE_SLOPE_SHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SHIFT` reader - "]
pub type LUT_LE_SLOPE_UFLOW_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SHIFT` writer - "]
pub type LUT_LE_SLOPE_UFLOW_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_LE_SLOPE_SHIFT_SPEC, u8, u8, 5, O>;
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SHIFT` reader - "]
pub type LUT_LE_SLOPE_OFLOW_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SHIFT` writer - "]
pub type LUT_LE_SLOPE_OFLOW_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_LE_SLOPE_SHIFT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_shift(&self) -> LUT_LE_SLOPE_UFLOW_SHIFT_R {
        LUT_LE_SLOPE_UFLOW_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_shift(&self) -> LUT_LE_SLOPE_OFLOW_SHIFT_R {
        LUT_LE_SLOPE_OFLOW_SHIFT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn lut_le_slope_uflow_shift(&mut self) -> LUT_LE_SLOPE_UFLOW_SHIFT_W<0> {
        LUT_LE_SLOPE_UFLOW_SHIFT_W::new(self)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn lut_le_slope_oflow_shift(&mut self) -> LUT_LE_SLOPE_OFLOW_SHIFT_W<5> {
        LUT_LE_SLOPE_OFLOW_SHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope shift parameter for LE_LUT underflow and overflow, signed value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_le_slope_shift](index.html) module"]
pub struct S_LUT_LE_SLOPE_SHIFT_SPEC;
impl crate::RegisterSpec for S_LUT_LE_SLOPE_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_le_slope_shift::R](R) reader structure"]
impl crate::Readable for S_LUT_LE_SLOPE_SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_le_slope_shift::W](W) writer structure"]
impl crate::Writable for S_LUT_LE_SLOPE_SHIFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_LE_SLOPE_SHIFT to value 0"]
impl crate::Resettable for S_LUT_LE_SLOPE_SHIFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
