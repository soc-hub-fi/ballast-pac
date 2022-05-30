#[doc = "Register `S_LUT_LO_SLOPE_SHIFT` reader"]
pub struct R(crate::R<S_LUT_LO_SLOPE_SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LO_SLOPE_SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LO_SLOPE_SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LO_SLOPE_SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SHIFT` reader - "]
pub struct LUT_LO_SLOPE_UFLOW_SHIFT_R(crate::FieldReader<u8>);
impl LUT_LO_SLOPE_UFLOW_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LO_SLOPE_UFLOW_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LO_SLOPE_UFLOW_SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SHIFT` reader - "]
pub struct LUT_LO_SLOPE_OFLOW_SHIFT_R(crate::FieldReader<u8>);
impl LUT_LO_SLOPE_OFLOW_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LO_SLOPE_OFLOW_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LO_SLOPE_OFLOW_SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lut_lo_slope_uflow_shift(&self) -> LUT_LO_SLOPE_UFLOW_SHIFT_R {
        LUT_LO_SLOPE_UFLOW_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn lut_lo_slope_oflow_shift(&self) -> LUT_LO_SLOPE_OFLOW_SHIFT_R {
        LUT_LO_SLOPE_OFLOW_SHIFT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[doc = "Slope shift parameter for LO_LUT underflow and overflow, signed value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_lo_slope_shift](index.html) module"]
pub struct S_LUT_LO_SLOPE_SHIFT_SPEC;
impl crate::RegisterSpec for S_LUT_LO_SLOPE_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_lo_slope_shift::R](R) reader structure"]
impl crate::Readable for S_LUT_LO_SLOPE_SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_LUT_LO_SLOPE_SHIFT to value 0"]
impl crate::Resettable for S_LUT_LO_SLOPE_SHIFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
