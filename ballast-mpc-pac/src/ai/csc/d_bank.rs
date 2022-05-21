#[doc = "Register `D_BANK` reader"]
pub struct R(crate::R<D_BANK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_BANK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_BANK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_BANK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BANK` reader - "]
pub struct DATA_BANK_R(crate::FieldReader<u8, u8>);
impl DATA_BANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_BANK` reader - "]
pub struct WEIGHT_BANK_R(crate::FieldReader<u8, u8>);
impl WEIGHT_BANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WEIGHT_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_BANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn data_bank(&self) -> DATA_BANK_R {
        DATA_BANK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_bank(&self) -> WEIGHT_BANK_R {
        WEIGHT_BANK_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Number of data banks and weight banks in CBUF\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_bank](index.html) module"]
pub struct D_BANK_SPEC;
impl crate::RegisterSpec for D_BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_bank::R](R) reader structure"]
impl crate::Readable for D_BANK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_BANK to value 0"]
impl crate::Resettable for D_BANK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
