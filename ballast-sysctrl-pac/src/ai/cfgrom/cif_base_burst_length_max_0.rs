#[doc = "Register `CIF_BASE_BURST_LENGTH_MAX_0` reader"]
pub struct R(crate::R<CIF_BASE_BURST_LENGTH_MAX_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIF_BASE_BURST_LENGTH_MAX_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIF_BASE_BURST_LENGTH_MAX_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIF_BASE_BURST_LENGTH_MAX_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BASE_BURST_LENGTH_MAX` reader - "]
pub struct BASE_BURST_LENGTH_MAX_R(crate::FieldReader<u32, u32>);
impl BASE_BURST_LENGTH_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BASE_BURST_LENGTH_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_BURST_LENGTH_MAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn base_burst_length_max(&self) -> BASE_BURST_LENGTH_MAX_R {
        BASE_BURST_LENGTH_MAX_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cif_base_burst_length_max_0](index.html) module"]
pub struct CIF_BASE_BURST_LENGTH_MAX_0_SPEC;
impl crate::RegisterSpec for CIF_BASE_BURST_LENGTH_MAX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cif_base_burst_length_max_0::R](R) reader structure"]
impl crate::Readable for CIF_BASE_BURST_LENGTH_MAX_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIF_BASE_BURST_LENGTH_MAX_0 to value 0x80"]
impl crate::Resettable for CIF_BASE_BURST_LENGTH_MAX_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
