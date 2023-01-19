#[doc = "Register `INTSTATUS_32_63` reader"]
pub struct R(crate::R<INTSTATUS_32_63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTATUS_32_63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTATUS_32_63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTATUS_32_63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSTATUS` reader - "]
pub type INTSTATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new(self.bits)
    }
}
#[doc = "Bit 31 - 0 INTSTATUS (R) GPIO\\[63:32\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus_32_63](index.html) module"]
pub struct INTSTATUS_32_63_SPEC;
impl crate::RegisterSpec for INTSTATUS_32_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstatus_32_63::R](R) reader structure"]
impl crate::Readable for INTSTATUS_32_63_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTATUS_32_63 to value 0"]
impl crate::Resettable for INTSTATUS_32_63_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
