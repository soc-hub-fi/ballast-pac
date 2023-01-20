#[doc = "Register `BOOTSEL` reader"]
pub struct R(crate::R<BOOTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `bootsel_pad` reader - "]
pub type BOOTSEL_PAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bootsel_pad(&self) -> BOOTSEL_PAD_R {
        BOOTSEL_PAD_R::new(self.bits)
    }
}
#[doc = "Boot Sel value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootsel](index.html) module"]
pub struct BOOTSEL_SPEC;
impl crate::RegisterSpec for BOOTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootsel::R](R) reader structure"]
impl crate::Readable for BOOTSEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BOOTSEL to value 0"]
impl crate::Resettable for BOOTSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
