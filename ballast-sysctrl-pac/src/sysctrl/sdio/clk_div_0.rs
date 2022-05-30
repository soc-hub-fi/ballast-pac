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
pub struct DIV_COUNT_R(crate::FieldReader<u8>);
impl DIV_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_COUNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Div_count` writer - "]
pub struct DIV_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Valid` reader - "]
pub struct VALID_R(crate::FieldReader<bool>);
impl VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Valid` writer - "]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
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
    pub fn div_count(&mut self) -> DIV_COUNT_W {
        DIV_COUNT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
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
}
#[doc = "`reset()` method sets CLK_DIV_0 to value 0"]
impl crate::Resettable for CLK_DIV_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
