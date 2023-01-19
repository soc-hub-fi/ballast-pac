#[doc = "Register `TTA_PLL_ENABLE` reader"]
pub struct R(crate::R<TTA_PLL_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTA_PLL_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTA_PLL_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTA_PLL_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTA_PLL_ENABLE` writer"]
pub struct W(crate::W<TTA_PLL_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTA_PLL_ENABLE_SPEC>;
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
impl From<crate::W<TTA_PLL_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTA_PLL_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Enable` reader - "]
pub type ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Enable` writer - "]
pub type ENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTA_PLL_ENABLE_SPEC, u8, u8, 8, O>;
#[doc = "Field `Valid` reader - "]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `Valid` writer - "]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTA_PLL_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<23> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tta_pll_enable](index.html) module"]
pub struct TTA_PLL_ENABLE_SPEC;
impl crate::RegisterSpec for TTA_PLL_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tta_pll_enable::R](R) reader structure"]
impl crate::Readable for TTA_PLL_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tta_pll_enable::W](W) writer structure"]
impl crate::Writable for TTA_PLL_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTA_PLL_ENABLE to value 0"]
impl crate::Resettable for TTA_PLL_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
