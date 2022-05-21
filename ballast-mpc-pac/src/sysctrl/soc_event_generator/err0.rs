#[doc = "Register `ERR0` reader"]
pub struct R(crate::R<ERR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR0` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub struct ERR0_R(crate::FieldReader<u32, u32>);
impl ERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new(self.bits)
    }
}
#[doc = "Events 0-31 event queue overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err0](index.html) module"]
pub struct ERR0_SPEC;
impl crate::RegisterSpec for ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err0::R](R) reader structure"]
impl crate::Readable for ERR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR0 to value 0"]
impl crate::Resettable for ERR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
