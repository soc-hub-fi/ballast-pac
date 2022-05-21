#[doc = "Register `REG_PERIPH_CLK_DIV` reader"]
pub struct R(crate::R<REG_PERIPH_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_PERIPH_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_PERIPH_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_PERIPH_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_PERIPH_CLK_DIV` writer"]
pub struct W(crate::W<REG_PERIPH_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_PERIPH_CLK_DIV_SPEC>;
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
impl From<crate::W<REG_PERIPH_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_PERIPH_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIPH_CLK_DIV` reader - "]
pub struct PERIPH_CLK_DIV_R(crate::FieldReader<u16, u16>);
impl PERIPH_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PERIPH_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPH_CLK_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIPH_CLK_DIV` writer - "]
pub struct PERIPH_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn periph_clk_div(&self) -> PERIPH_CLK_DIV_R {
        PERIPH_CLK_DIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn periph_clk_div(&mut self) -> PERIPH_CLK_DIV_W {
        PERIPH_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_periph_clk_div](index.html) module"]
pub struct REG_PERIPH_CLK_DIV_SPEC;
impl crate::RegisterSpec for REG_PERIPH_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_periph_clk_div::R](R) reader structure"]
impl crate::Readable for REG_PERIPH_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_periph_clk_div::W](W) writer structure"]
impl crate::Writable for REG_PERIPH_CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_PERIPH_CLK_DIV to value 0"]
impl crate::Resettable for REG_PERIPH_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
