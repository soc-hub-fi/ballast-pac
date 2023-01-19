#[doc = "Register `INTER_PLL_DEBUG_CTRL` reader"]
pub struct R(crate::R<INTER_PLL_DEBUG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTER_PLL_DEBUG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTER_PLL_DEBUG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTER_PLL_DEBUG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTER_PLL_DEBUG_CTRL` writer"]
pub struct W(crate::W<INTER_PLL_DEBUG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTER_PLL_DEBUG_CTRL_SPEC>;
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
impl From<crate::W<INTER_PLL_DEBUG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTER_PLL_DEBUG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Debug_Ctrl` reader - "]
pub type DEBUG_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Debug_Ctrl` writer - "]
pub type DEBUG_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTER_PLL_DEBUG_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn debug_ctrl(&self) -> DEBUG_CTRL_R {
        DEBUG_CTRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ctrl(&mut self) -> DEBUG_CTRL_W<0> {
        DEBUG_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inter_pll_debug_ctrl](index.html) module"]
pub struct INTER_PLL_DEBUG_CTRL_SPEC;
impl crate::RegisterSpec for INTER_PLL_DEBUG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inter_pll_debug_ctrl::R](R) reader structure"]
impl crate::Readable for INTER_PLL_DEBUG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inter_pll_debug_ctrl::W](W) writer structure"]
impl crate::Writable for INTER_PLL_DEBUG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTER_PLL_DEBUG_CTRL to value 0"]
impl crate::Resettable for INTER_PLL_DEBUG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
