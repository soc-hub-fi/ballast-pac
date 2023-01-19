#[doc = "Register `PULP_PLL_DIV` reader"]
pub struct R(crate::R<PULP_PLL_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULP_PLL_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULP_PLL_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULP_PLL_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULP_PLL_DIV` writer"]
pub struct W(crate::W<PULP_PLL_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULP_PLL_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PULP_PLL_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULP_PLL_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `r_div` reader - "]
pub type R_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `r_div` writer - "]
pub type R_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULP_PLL_DIV_SPEC, u8, u8, 4, O>;
#[doc = "Field `n_div` reader - "]
pub type N_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `n_div` writer - "]
pub type N_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULP_PLL_DIV_SPEC, u16, u16, 10, O>;
#[doc = "Field `m_div` reader - "]
pub type M_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `m_div` writer - "]
pub type M_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULP_PLL_DIV_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_div(&self) -> R_DIV_R {
        R_DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn n_div(&self) -> N_DIV_R {
        N_DIV_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn m_div(&self) -> M_DIV_R {
        M_DIV_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn r_div(&mut self) -> R_DIV_W<0> {
        R_DIV_W::new(self)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    #[must_use]
    pub fn n_div(&mut self) -> N_DIV_W<4> {
        N_DIV_W::new(self)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    #[must_use]
    pub fn m_div(&mut self) -> M_DIV_W<14> {
        M_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulp_pll_div](index.html) module"]
pub struct PULP_PLL_DIV_SPEC;
impl crate::RegisterSpec for PULP_PLL_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulp_pll_div::R](R) reader structure"]
impl crate::Readable for PULP_PLL_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulp_pll_div::W](W) writer structure"]
impl crate::Writable for PULP_PLL_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULP_PLL_DIV to value 0"]
impl crate::Resettable for PULP_PLL_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
