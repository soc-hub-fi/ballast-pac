#[doc = "Register `PADIN_32_63` reader"]
pub struct R(crate::R<PADIN_32_63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADIN_32_63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADIN_32_63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADIN_32_63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_IN` reader - Bit 31 - 0 DATA_IN (R) GPIO\\[63:32\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
pub struct DATA_IN_R(crate::FieldReader<u32>);
impl DATA_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_IN_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Bit 31 - 0 DATA_IN (R) GPIO\\[63:32\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
    #[inline(always)]
    pub fn data_in(&self) -> DATA_IN_R {
        DATA_IN_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padin_32_63](index.html) module"]
pub struct PADIN_32_63_SPEC;
impl crate::RegisterSpec for PADIN_32_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padin_32_63::R](R) reader structure"]
impl crate::Readable for PADIN_32_63_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PADIN_32_63 to value 0"]
impl crate::Resettable for PADIN_32_63_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
