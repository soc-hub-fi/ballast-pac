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
#[doc = "Slope scale parameter for LE LUT underflow and overflow, signed value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_le_slope_scale](index.html) module"]
pub struct S_LUT_LE_SLOPE_SCALE_SPEC;
impl crate::RegisterSpec for S_LUT_LE_SLOPE_SCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_le_slope_scale::R](R) reader structure"]
impl crate::Readable for S_LUT_LE_SLOPE_SCALE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_LUT_LE_SLOPE_SCALE to value 0"]
impl crate::Resettable for S_LUT_LE_SLOPE_SCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
