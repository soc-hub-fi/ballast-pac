#[doc = "Register `CLK_DIV_0` reader"]
pub struct R(crate::R<CLK_DIV_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DIV_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DIV_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DIV_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DIV_0` writer"]
pub struct W(crate::W<CLK_DIV_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DIV_0_SPEC>;
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
impl From<crate::W<CLK_DIV_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DIV_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Div_count` reader - "]
pub type DIV_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Div_count` writer - "]
pub type DIV_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_DIV_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `Valid` reader - "]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `Valid` writer - "]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_DIV_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_count(&self) -> DIV_COUNT_R {
        DIV_COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn div_count(&mut self) -> DIV_COUNT_W<0> {
        DIV_COUNT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<8> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div_0](index.html) module"]
pub struct CLK_DIV_0_SPEC;
impl crate::RegisterSpec for CLK_DIV_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_div_0::R](R) reader structure"]
impl crate::Readable for CLK_DIV_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_div_0::W](W) writer structure"]
impl crate::Writable for CLK_DIV_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_DIV_0 to value 0"]
impl crate::Resettable for CLK_DIV_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
