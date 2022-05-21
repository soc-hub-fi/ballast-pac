#[doc = "Register `CMAC_A_BASE_CDMA_ID_0` reader"]
pub struct R(crate::R<CMAC_A_BASE_CDMA_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAC_A_BASE_CDMA_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAC_A_BASE_CDMA_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAC_A_BASE_CDMA_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMAC_A_BASE_CDMA_ID` reader - "]
pub struct CMAC_A_BASE_CDMA_ID_R(crate::FieldReader<u32, u32>);
impl CMAC_A_BASE_CDMA_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMAC_A_BASE_CDMA_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMAC_A_BASE_CDMA_ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmac_a_base_cdma_id(&self) -> CMAC_A_BASE_CDMA_ID_R {
        CMAC_A_BASE_CDMA_ID_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmac_a_base_cdma_id_0](index.html) module"]
pub struct CMAC_A_BASE_CDMA_ID_0_SPEC;
impl crate::RegisterSpec for CMAC_A_BASE_CDMA_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmac_a_base_cdma_id_0::R](R) reader structure"]
impl crate::Readable for CMAC_A_BASE_CDMA_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMAC_A_BASE_CDMA_ID_0 to value 0x03"]
impl crate::Resettable for CMAC_A_BASE_CDMA_ID_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
