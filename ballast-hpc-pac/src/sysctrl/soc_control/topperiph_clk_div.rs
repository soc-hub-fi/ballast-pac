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
#[doc = "Field `REG_TOPPERIPH_CLK_DIV` reader - Clock divider ratio for Top peripheral module"]
pub struct REG_TOPPERIPH_CLK_DIV_R(crate::FieldReader<u16>);
impl REG_TOPPERIPH_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_TOPPERIPH_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_TOPPERIPH_CLK_DIV_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_TOPPERIPH_CLK_DIV` writer - Clock divider ratio for Top peripheral module"]
pub struct REG_TOPPERIPH_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_TOPPERIPH_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock divider ratio for Top peripheral module"]
    #[inline(always)]
    pub fn reg_topperiph_clk_div(&self) -> REG_TOPPERIPH_CLK_DIV_R {
        REG_TOPPERIPH_CLK_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock divider ratio for Top peripheral module"]
    #[inline(always)]
    pub fn reg_topperiph_clk_div(&mut self) -> REG_TOPPERIPH_CLK_DIV_W {
        REG_TOPPERIPH_CLK_DIV_W { w: self }
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
}
#[doc = "`reset()` method sets TOPPERIPH_CLK_DIV to value 0"]
impl crate::Resettable for TOPPERIPH_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
