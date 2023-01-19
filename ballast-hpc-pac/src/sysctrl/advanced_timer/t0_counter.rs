#[doc = "Register `T0_COUNTER` reader"]
pub struct R(crate::R<T0_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_COUNTER` reader - "]
pub type T0_COUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t0_counter(&self) -> T0_COUNTER_R {
        T0_COUNTER_R::new(self.bits)
    }
}
#[doc = "ADV_TIMER0 counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0_counter](index.html) module"]
pub struct T0_COUNTER_SPEC;
impl crate::RegisterSpec for T0_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0_counter::R](R) reader structure"]
impl crate::Readable for T0_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0_COUNTER to value 0"]
impl crate::Resettable for T0_COUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
