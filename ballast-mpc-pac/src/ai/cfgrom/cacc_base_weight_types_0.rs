#[doc = "Register `CACC_BASE_WEIGHT_TYPES_0` reader"]
pub struct R(crate::R<CACC_BASE_WEIGHT_TYPES_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACC_BASE_WEIGHT_TYPES_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACC_BASE_WEIGHT_TYPES_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACC_BASE_WEIGHT_TYPES_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACC_BASE_WEIGHT_TYPES` reader - "]
pub type CACC_BASE_WEIGHT_TYPES_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cacc_base_weight_types(&self) -> CACC_BASE_WEIGHT_TYPES_R {
        CACC_BASE_WEIGHT_TYPES_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacc_base_weight_types_0](index.html) module"]
pub struct CACC_BASE_WEIGHT_TYPES_0_SPEC;
impl crate::RegisterSpec for CACC_BASE_WEIGHT_TYPES_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacc_base_weight_types_0::R](R) reader structure"]
impl crate::Readable for CACC_BASE_WEIGHT_TYPES_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACC_BASE_WEIGHT_TYPES_0 to value 0x10"]
impl crate::Resettable for CACC_BASE_WEIGHT_TYPES_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
