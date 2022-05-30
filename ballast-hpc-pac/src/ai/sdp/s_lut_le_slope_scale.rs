#[doc = "Register `S_LUT_LE_SLOPE_SCALE` reader"]
pub struct R(crate::R<S_LUT_LE_SLOPE_SCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LE_SLOPE_SCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LE_SLOPE_SCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LE_SLOPE_SCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_LE_SLOPE_SCALE` writer"]
pub struct W(crate::W<S_LUT_LE_SLOPE_SCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_LE_SLOPE_SCALE_SPEC>;
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
impl From<crate::W<S_LUT_LE_SLOPE_SCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_LE_SLOPE_SCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SCALE` reader - "]
pub struct LUT_LE_SLOPE_UFLOW_SCALE_R(crate::FieldReader<u16>);
impl LUT_LE_SLOPE_UFLOW_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LUT_LE_SLOPE_UFLOW_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_SLOPE_UFLOW_SCALE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SCALE` writer - "]
pub struct LUT_LE_SLOPE_UFLOW_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LE_SLOPE_UFLOW_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SCALE` reader - "]
pub struct LUT_LE_SLOPE_OFLOW_SCALE_R(crate::FieldReader<u16>);
impl LUT_LE_SLOPE_OFLOW_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LUT_LE_SLOPE_OFLOW_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_SLOPE_OFLOW_SCALE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SCALE` writer - "]
pub struct LUT_LE_SLOPE_OFLOW_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LE_SLOPE_OFLOW_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_scale(&self) -> LUT_LE_SLOPE_UFLOW_SCALE_R {
        LUT_LE_SLOPE_UFLOW_SCALE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_scale(&self) -> LUT_LE_SLOPE_OFLOW_SCALE_R {
        LUT_LE_SLOPE_OFLOW_SCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_scale(&mut self) -> LUT_LE_SLOPE_UFLOW_SCALE_W {
        LUT_LE_SLOPE_UFLOW_SCALE_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_scale(&mut self) -> LUT_LE_SLOPE_OFLOW_SCALE_W {
        LUT_LE_SLOPE_OFLOW_SCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope scale parameter for LE LUT underflow and overflow, signed value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_le_slope_scale](index.html) module"]
pub struct S_LUT_LE_SLOPE_SCALE_SPEC;
impl crate::RegisterSpec for S_LUT_LE_SLOPE_SCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_le_slope_scale::R](R) reader structure"]
impl crate::Readable for S_LUT_LE_SLOPE_SCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_le_slope_scale::W](W) writer structure"]
impl crate::Writable for S_LUT_LE_SLOPE_SCALE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_LUT_LE_SLOPE_SCALE to value 0"]
impl crate::Resettable for S_LUT_LE_SLOPE_SCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
