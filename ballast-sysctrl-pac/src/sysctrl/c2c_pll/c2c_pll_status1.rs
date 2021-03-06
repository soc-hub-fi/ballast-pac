#[doc = "Register `C2C_PLL_STATUS1` reader"]
pub struct R(crate::R<C2C_PLL_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2C_PLL_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2C_PLL_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2C_PLL_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `status1` reader - "]
pub struct STATUS1_R(crate::FieldReader<u32>);
impl STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS1_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status1(&self) -> STATUS1_R {
        STATUS1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2c_pll_status1](index.html) module"]
pub struct C2C_PLL_STATUS1_SPEC;
impl crate::RegisterSpec for C2C_PLL_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2c_pll_status1::R](R) reader structure"]
impl crate::Readable for C2C_PLL_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C2C_PLL_STATUS1 to value 0"]
impl crate::Resettable for C2C_PLL_STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
