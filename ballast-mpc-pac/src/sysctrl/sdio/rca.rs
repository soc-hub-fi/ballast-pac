#[doc = "Register `RCA` reader"]
pub struct R(crate::R<RCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCA` reader - "]
pub struct RCA_R(crate::FieldReader<u16>);
impl RCA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCA_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rca(&self) -> RCA_R {
        RCA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rca](index.html) module"]
pub struct RCA_SPEC;
impl crate::RegisterSpec for RCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rca::R](R) reader structure"]
impl crate::Readable for RCA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCA to value 0"]
impl crate::Resettable for RCA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
