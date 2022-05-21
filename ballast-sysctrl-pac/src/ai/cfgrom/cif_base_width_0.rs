#[doc = "Register `CIF_BASE_WIDTH_0` reader"]
pub struct R(crate::R<CIF_BASE_WIDTH_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIF_BASE_WIDTH_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIF_BASE_WIDTH_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIF_BASE_WIDTH_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIF_BASE_WIDTH` reader - "]
pub struct CIF_BASE_WIDTH_R(crate::FieldReader<u8, u8>);
impl CIF_BASE_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CIF_BASE_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIF_BASE_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cif_base_width(&self) -> CIF_BASE_WIDTH_R {
        CIF_BASE_WIDTH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cif_base_width_0](index.html) module"]
pub struct CIF_BASE_WIDTH_0_SPEC;
impl crate::RegisterSpec for CIF_BASE_WIDTH_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cif_base_width_0::R](R) reader structure"]
impl crate::Readable for CIF_BASE_WIDTH_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIF_BASE_WIDTH_0 to value 0x08"]
impl crate::Resettable for CIF_BASE_WIDTH_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
