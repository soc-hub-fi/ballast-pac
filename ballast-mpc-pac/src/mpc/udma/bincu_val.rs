#[doc = "Register `BINCU_VAL` reader"]
pub struct R(crate::R<BINCU_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINCU_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINCU_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINCU_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BINCU_VAL` reader - "]
pub struct BINCU_VAL_R(crate::FieldReader<u32>);
impl BINCU_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BINCU_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BINCU_VAL_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bincu_val(&self) -> BINCU_VAL_R {
        BINCU_VAL_R::new(self.bits)
    }
}
#[doc = "FILTER binarization result count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bincu_val](index.html) module"]
pub struct BINCU_VAL_SPEC;
impl crate::RegisterSpec for BINCU_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bincu_val::R](R) reader structure"]
impl crate::Readable for BINCU_VAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BINCU_VAL to value 0"]
impl crate::Resettable for BINCU_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
