#[doc = "Register `D_PARTIAL_WIDTH_OUT` reader"]
pub struct R(crate::R<D_PARTIAL_WIDTH_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PARTIAL_WIDTH_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PARTIAL_WIDTH_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PARTIAL_WIDTH_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTIAL_WIDTH_OUT_FIRST` reader - "]
pub struct PARTIAL_WIDTH_OUT_FIRST_R(crate::FieldReader<u16, u16>);
impl PARTIAL_WIDTH_OUT_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTIAL_WIDTH_OUT_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTIAL_WIDTH_OUT_FIRST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTIAL_WIDTH_OUT_LAST` reader - "]
pub struct PARTIAL_WIDTH_OUT_LAST_R(crate::FieldReader<u16, u16>);
impl PARTIAL_WIDTH_OUT_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTIAL_WIDTH_OUT_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTIAL_WIDTH_OUT_LAST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTIAL_WIDTH_OUT_MID` reader - "]
pub struct PARTIAL_WIDTH_OUT_MID_R(crate::FieldReader<u16, u16>);
impl PARTIAL_WIDTH_OUT_MID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTIAL_WIDTH_OUT_MID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTIAL_WIDTH_OUT_MID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn partial_width_out_first(&self) -> PARTIAL_WIDTH_OUT_FIRST_R {
        PARTIAL_WIDTH_OUT_FIRST_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn partial_width_out_last(&self) -> PARTIAL_WIDTH_OUT_LAST_R {
        PARTIAL_WIDTH_OUT_LAST_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn partial_width_out_mid(&self) -> PARTIAL_WIDTH_OUT_MID_R {
        PARTIAL_WIDTH_OUT_MID_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[doc = "Partial width for first, last and middle partitions of output cube\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_partial_width_out](index.html) module"]
pub struct D_PARTIAL_WIDTH_OUT_SPEC;
impl crate::RegisterSpec for D_PARTIAL_WIDTH_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_partial_width_out::R](R) reader structure"]
impl crate::Readable for D_PARTIAL_WIDTH_OUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PARTIAL_WIDTH_OUT to value 0"]
impl crate::Resettable for D_PARTIAL_WIDTH_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
