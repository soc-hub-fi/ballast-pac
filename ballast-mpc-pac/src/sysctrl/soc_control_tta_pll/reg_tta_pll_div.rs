#[doc = "Register `REG_TTA_PLL_DIV` reader"]
pub struct R(crate::R<REG_TTA_PLL_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_TTA_PLL_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_TTA_PLL_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_TTA_PLL_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_TTA_PLL_DIV` writer"]
pub struct W(crate::W<REG_TTA_PLL_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_TTA_PLL_DIV_SPEC>;
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
impl From<crate::W<REG_TTA_PLL_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_TTA_PLL_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `r_div` reader - "]
pub struct R_DIV_R(crate::FieldReader<u8, u8>);
impl R_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `r_div` writer - "]
pub struct R_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> R_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `n_div` reader - "]
pub struct N_DIV_R(crate::FieldReader<u16, u16>);
impl N_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        N_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `n_div` writer - "]
pub struct N_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> N_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 4)) | ((value as u32 & 0x03ff) << 4);
        self.w
    }
}
#[doc = "Field `m_div` reader - "]
pub struct M_DIV_R(crate::FieldReader<u8, u8>);
impl M_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `m_div` writer - "]
pub struct M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> M_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 14)) | ((value as u32 & 7) << 14);
        self.w
    }
}
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
    pub fn r_div(&mut self) -> R_DIV_W {
        R_DIV_W { w: self }
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn n_div(&mut self) -> N_DIV_W {
        N_DIV_W { w: self }
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn m_div(&mut self) -> M_DIV_W {
        M_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_tta_pll_div](index.html) module"]
pub struct REG_TTA_PLL_DIV_SPEC;
impl crate::RegisterSpec for REG_TTA_PLL_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_tta_pll_div::R](R) reader structure"]
impl crate::Readable for REG_TTA_PLL_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_tta_pll_div::W](W) writer structure"]
impl crate::Writable for REG_TTA_PLL_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_TTA_PLL_DIV to value 0"]
impl crate::Resettable for REG_TTA_PLL_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
