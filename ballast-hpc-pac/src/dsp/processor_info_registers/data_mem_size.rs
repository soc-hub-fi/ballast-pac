#[doc = "Register `data_mem_size` reader"]
pub struct R(crate::R<DATA_MEM_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_MEM_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_MEM_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_MEM_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `data_mem_size` reader - "]
pub struct DATA_MEM_SIZE_R(crate::FieldReader<u32>);
impl DATA_MEM_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_MEM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_MEM_SIZE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_mem_size(&self) -> DATA_MEM_SIZE_R {
        DATA_MEM_SIZE_R::new(self.bits)
    }
}
#[doc = "Data memory size, in bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_mem_size](index.html) module"]
pub struct DATA_MEM_SIZE_SPEC;
impl crate::RegisterSpec for DATA_MEM_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_mem_size::R](R) reader structure"]
impl crate::Readable for DATA_MEM_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets data_mem_size to value 0"]
impl crate::Resettable for DATA_MEM_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
