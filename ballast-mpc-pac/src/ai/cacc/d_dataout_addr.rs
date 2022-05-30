#[doc = "Register `D_DATAOUT_ADDR` reader"]
pub struct R(crate::R<D_DATAOUT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAOUT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAOUT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAOUT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAOUT_ADDR` reader - "]
pub struct DATAOUT_ADDR_R(crate::FieldReader<u32>);
impl DATAOUT_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATAOUT_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAOUT_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dataout_addr(&self) -> DATAOUT_ADDR_R {
        DATAOUT_ADDR_R::new(self.bits)
    }
}
#[doc = "Address of output cube\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dataout_addr](index.html) module"]
pub struct D_DATAOUT_ADDR_SPEC;
impl crate::RegisterSpec for D_DATAOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dataout_addr::R](R) reader structure"]
impl crate::Readable for D_DATAOUT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATAOUT_ADDR to value 0"]
impl crate::Resettable for D_DATAOUT_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
