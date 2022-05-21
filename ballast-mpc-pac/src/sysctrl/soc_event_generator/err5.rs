#[doc = "Register `ERR5` reader"]
pub struct R(crate::R<ERR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR5` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub struct ERR5_R(crate::FieldReader<u32, u32>);
impl ERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR5_R {
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
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(self.bits)
    }
}
#[doc = "Events 160-191 event queue overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err5](index.html) module"]
pub struct ERR5_SPEC;
impl crate::RegisterSpec for ERR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err5::R](R) reader structure"]
impl crate::Readable for ERR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR5 to value 0"]
impl crate::Resettable for ERR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
