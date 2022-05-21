#[doc = "Register `REG_TTA_PLL_STATUS2` reader"]
pub struct R(crate::R<REG_TTA_PLL_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_TTA_PLL_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_TTA_PLL_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_TTA_PLL_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `status2` reader - "]
pub struct STATUS2_R(crate::FieldReader<u32, u32>);
impl STATUS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STATUS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status2(&self) -> STATUS2_R {
        STATUS2_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_tta_pll_status2](index.html) module"]
pub struct REG_TTA_PLL_STATUS2_SPEC;
impl crate::RegisterSpec for REG_TTA_PLL_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_tta_pll_status2::R](R) reader structure"]
impl crate::Readable for REG_TTA_PLL_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REG_TTA_PLL_STATUS2 to value 0"]
impl crate::Resettable for REG_TTA_PLL_STATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
