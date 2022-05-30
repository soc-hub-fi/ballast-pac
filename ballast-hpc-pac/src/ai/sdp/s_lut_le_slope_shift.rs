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
pub struct LUT_LE_SLOPE_UFLOW_SHIFT_R(crate::FieldReader<u8>);
impl LUT_LE_SLOPE_UFLOW_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LE_SLOPE_UFLOW_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_SLOPE_UFLOW_SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SHIFT` writer - "]
pub struct LUT_LE_SLOPE_UFLOW_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LE_SLOPE_UFLOW_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SHIFT` reader - "]
pub struct LUT_LE_SLOPE_OFLOW_SHIFT_R(crate::FieldReader<u8>);
impl LUT_LE_SLOPE_OFLOW_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LE_SLOPE_OFLOW_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_SLOPE_OFLOW_SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SHIFT` writer - "]
pub struct LUT_LE_SLOPE_OFLOW_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LE_SLOPE_OFLOW_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
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
    pub fn lut_le_slope_uflow_shift(&mut self) -> LUT_LE_SLOPE_UFLOW_SHIFT_W {
        LUT_LE_SLOPE_UFLOW_SHIFT_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_shift(&mut self) -> LUT_LE_SLOPE_OFLOW_SHIFT_W {
        LUT_LE_SLOPE_OFLOW_SHIFT_W { w: self }
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
}
#[doc = "`reset()` method sets S_LUT_LE_SLOPE_SHIFT to value 0"]
impl crate::Resettable for S_LUT_LE_SLOPE_SHIFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
