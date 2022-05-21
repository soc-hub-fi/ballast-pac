#[doc = "Register `T3_COUNTER` reader"]
pub struct R(crate::R<T3_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T3_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T3_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T3_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T3_COUNTER` reader - "]
pub struct T3_COUNTER_R(crate::FieldReader<u32, u32>);
impl T3_COUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T3_COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3_COUNTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t3_counter(&self) -> T3_COUNTER_R {
        T3_COUNTER_R::new(self.bits)
    }
}
#[doc = "ADV_TIMER3 counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3_counter](index.html) module"]
pub struct T3_COUNTER_SPEC;
impl crate::RegisterSpec for T3_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t3_counter::R](R) reader structure"]
impl crate::Readable for T3_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T3_COUNTER to value 0"]
impl crate::Resettable for T3_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
