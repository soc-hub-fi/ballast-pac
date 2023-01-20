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
pub type CTRL_SIZE_R = crate::FieldReader<u32, u32>;
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
    const RESET_VALUE: Self::Ux = 0;
}
