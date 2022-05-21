#[doc = "Register `D_EW_LINE_STRIDE` reader"]
pub struct R(crate::R<D_EW_LINE_STRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_EW_LINE_STRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_EW_LINE_STRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_EW_LINE_STRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EW_LINE_STRIDE` reader - "]
pub struct EW_LINE_STRIDE_R(crate::FieldReader<u32, u32>);
impl EW_LINE_STRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EW_LINE_STRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_LINE_STRIDE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ew_line_stride(&self) -> EW_LINE_STRIDE_R {
        EW_LINE_STRIDE_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_ew_line_stride](index.html) module"]
pub struct D_EW_LINE_STRIDE_SPEC;
impl crate::RegisterSpec for D_EW_LINE_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_ew_line_stride::R](R) reader structure"]
impl crate::Readable for D_EW_LINE_STRIDE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_EW_LINE_STRIDE to value 0"]
impl crate::Resettable for D_EW_LINE_STRIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
