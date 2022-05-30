#[doc = "Register `instr_mem_size` reader"]
pub struct R(crate::R<INSTR_MEM_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTR_MEM_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTR_MEM_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTR_MEM_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `instr_mem_size` reader - "]
pub struct INSTR_MEM_SIZE_R(crate::FieldReader<u32>);
impl INSTR_MEM_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INSTR_MEM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTR_MEM_SIZE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn instr_mem_size(&self) -> INSTR_MEM_SIZE_R {
        INSTR_MEM_SIZE_R::new(self.bits)
    }
}
#[doc = "Instruction memory size, in bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_mem_size](index.html) module"]
pub struct INSTR_MEM_SIZE_SPEC;
impl crate::RegisterSpec for INSTR_MEM_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instr_mem_size::R](R) reader structure"]
impl crate::Readable for INSTR_MEM_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets instr_mem_size to value 0"]
impl crate::Resettable for INSTR_MEM_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
