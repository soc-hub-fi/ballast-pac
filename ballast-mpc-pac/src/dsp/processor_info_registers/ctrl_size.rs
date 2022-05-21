#[doc = "Register `ctrl_size` reader"]
pub struct R(crate::R<CTRL_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ctrl_size` reader - "]
pub struct CTRL_SIZE_R(crate::FieldReader<u32, u32>);
impl CTRL_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CTRL_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_SIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ctrl_size(&self) -> CTRL_SIZE_R {
        CTRL_SIZE_R::new(self.bits)
    }
}
#[doc = "CTRL size, per core, in bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_size](index.html) module"]
pub struct CTRL_SIZE_SPEC;
impl crate::RegisterSpec for CTRL_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_size::R](R) reader structure"]
impl crate::Readable for CTRL_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ctrl_size to value 0"]
impl crate::Resettable for CTRL_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
