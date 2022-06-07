#[doc = "Register `PULP_PLL_DEBUG_CTRL` reader"]
pub struct R(crate::R<PULP_PLL_DEBUG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULP_PLL_DEBUG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULP_PLL_DEBUG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULP_PLL_DEBUG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULP_PLL_DEBUG_CTRL` writer"]
pub struct W(crate::W<PULP_PLL_DEBUG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULP_PLL_DEBUG_CTRL_SPEC>;
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
impl From<crate::W<PULP_PLL_DEBUG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULP_PLL_DEBUG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Debug_Ctrl` reader - "]
pub struct DEBUG_CTRL_R(crate::FieldReader<u8>);
impl DEBUG_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_CTRL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Debug_Ctrl` writer - "]
pub struct DEBUG_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn debug_ctrl(&mut self) -> DEBUG_CTRL_W {
        DEBUG_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulp_pll_debug_ctrl](index.html) module"]
pub struct PULP_PLL_DEBUG_CTRL_SPEC;
impl crate::RegisterSpec for PULP_PLL_DEBUG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulp_pll_debug_ctrl::R](R) reader structure"]
impl crate::Readable for PULP_PLL_DEBUG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulp_pll_debug_ctrl::W](W) writer structure"]
impl crate::Writable for PULP_PLL_DEBUG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PULP_PLL_DEBUG_CTRL to value 0"]
impl crate::Resettable for PULP_PLL_DEBUG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
