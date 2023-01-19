#[doc = "Register `software_irq_read` reader"]
pub struct R(crate::R<SOFTWARE_IRQ_READ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTWARE_IRQ_READ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTWARE_IRQ_READ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTWARE_IRQ_READ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `read` reader - "]
pub type READ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [software_irq_read](index.html) module"]
pub struct SOFTWARE_IRQ_READ_SPEC;
impl crate::RegisterSpec for SOFTWARE_IRQ_READ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [software_irq_read::R](R) reader structure"]
impl crate::Readable for SOFTWARE_IRQ_READ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets software_irq_read to value 0"]
impl crate::Resettable for SOFTWARE_IRQ_READ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
