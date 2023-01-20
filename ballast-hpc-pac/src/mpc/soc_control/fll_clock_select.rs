#[doc = "Register `FLL_CLOCK_SELECT` reader"]
pub struct R(crate::R<FLL_CLOCK_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLL_CLOCK_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLL_CLOCK_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLL_CLOCK_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `S` reader - "]
pub type S_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new((self.bits & 1) != 0)
    }
}
#[doc = "This register contains whether the system clock is coming from the FLL or the FLL is bypassed. It is a read-only register by the core but it can be written via JTAG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fll_clock_select](index.html) module"]
pub struct FLL_CLOCK_SELECT_SPEC;
impl crate::RegisterSpec for FLL_CLOCK_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fll_clock_select::R](R) reader structure"]
impl crate::Readable for FLL_CLOCK_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLL_CLOCK_SELECT to value 0"]
impl crate::Resettable for FLL_CLOCK_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
