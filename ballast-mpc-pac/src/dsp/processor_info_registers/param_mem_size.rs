#[doc = "Register `param_mem_size` reader"]
pub struct R(crate::R<PARAM_MEM_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_MEM_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_MEM_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_MEM_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `param_mem_size` reader - "]
pub type PARAM_MEM_SIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn param_mem_size(&self) -> PARAM_MEM_SIZE_R {
        PARAM_MEM_SIZE_R::new(self.bits)
    }
}
#[doc = "Parameter memory size, in bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param_mem_size](index.html) module"]
pub struct PARAM_MEM_SIZE_SPEC;
impl crate::RegisterSpec for PARAM_MEM_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param_mem_size::R](R) reader structure"]
impl crate::Readable for PARAM_MEM_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets param_mem_size to value 0"]
impl crate::Resettable for PARAM_MEM_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
