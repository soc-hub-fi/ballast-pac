#[doc = "Register `CACC_DESC_0` reader"]
pub struct R(crate::R<CACC_DESC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACC_DESC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACC_DESC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACC_DESC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACC_DESC` reader - "]
pub type CACC_DESC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cacc_desc(&self) -> CACC_DESC_R {
        CACC_DESC_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacc_desc_0](index.html) module"]
pub struct CACC_DESC_0_SPEC;
impl crate::RegisterSpec for CACC_DESC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacc_desc_0::R](R) reader structure"]
impl crate::Readable for CACC_DESC_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACC_DESC_0 to value 0x0020_0007"]
impl crate::Resettable for CACC_DESC_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0007;
}
