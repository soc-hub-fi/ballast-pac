#[doc = "Register `TOPPERIPH_CLK_DIV` reader"]
pub struct R(crate::R<TOPPERIPH_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOPPERIPH_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOPPERIPH_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOPPERIPH_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOPPERIPH_CLK_DIV` writer"]
pub struct W(crate::W<TOPPERIPH_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOPPERIPH_CLK_DIV_SPEC>;
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
impl From<crate::W<TOPPERIPH_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOPPERIPH_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOPPERIPH_CLK_DIV` reader - Clock divider ratio for Top peripheral module"]
pub type TOPPERIPH_CLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOPPERIPH_CLK_DIV` writer - Clock divider ratio for Top peripheral module"]
pub type TOPPERIPH_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOPPERIPH_CLK_DIV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Clock divider ratio for Top peripheral module"]
    #[inline(always)]
    pub fn topperiph_clk_div(&self) -> TOPPERIPH_CLK_DIV_R {
        TOPPERIPH_CLK_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock divider ratio for Top peripheral module"]
    #[inline(always)]
    #[must_use]
    pub fn topperiph_clk_div(&mut self) -> TOPPERIPH_CLK_DIV_W<0> {
        TOPPERIPH_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [topperiph_clk_div](index.html) module"]
pub struct TOPPERIPH_CLK_DIV_SPEC;
impl crate::RegisterSpec for TOPPERIPH_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [topperiph_clk_div::R](R) reader structure"]
impl crate::Readable for TOPPERIPH_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [topperiph_clk_div::W](W) writer structure"]
impl crate::Writable for TOPPERIPH_CLK_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOPPERIPH_CLK_DIV to value 0"]
impl crate::Resettable for TOPPERIPH_CLK_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
