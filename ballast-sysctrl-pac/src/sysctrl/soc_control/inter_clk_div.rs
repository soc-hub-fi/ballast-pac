#[doc = "Register `INTER_CLK_DIV` reader"]
pub struct R(crate::R<INTER_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTER_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTER_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTER_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTER_CLK_DIV` writer"]
pub struct W(crate::W<INTER_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTER_CLK_DIV_SPEC>;
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
impl From<crate::W<INTER_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTER_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFGICN_DIV` reader - "]
pub struct CFGICN_DIV_R(crate::FieldReader<u8>);
impl CFGICN_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFGICN_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGICN_DIV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGICN_DIV` writer - "]
pub struct CFGICN_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGICN_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LPICN_DIV` reader - "]
pub struct LPICN_DIV_R(crate::FieldReader<u8>);
impl LPICN_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPICN_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPICN_DIV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPICN_DIV` writer - "]
pub struct LPICN_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPICN_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HPICN_DIV` reader - "]
pub struct HPICN_DIV_R(crate::FieldReader<u8>);
impl HPICN_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HPICN_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPICN_DIV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPICN_DIV` writer - "]
pub struct HPICN_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HPICN_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfgicn_div(&self) -> CFGICN_DIV_R {
        CFGICN_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lpicn_div(&self) -> LPICN_DIV_R {
        LPICN_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hpicn_div(&self) -> HPICN_DIV_R {
        HPICN_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfgicn_div(&mut self) -> CFGICN_DIV_W {
        CFGICN_DIV_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lpicn_div(&mut self) -> LPICN_DIV_W {
        LPICN_DIV_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hpicn_div(&mut self) -> HPICN_DIV_W {
        HPICN_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divider ratio for the 3 Interconnect modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inter_clk_div](index.html) module"]
pub struct INTER_CLK_DIV_SPEC;
impl crate::RegisterSpec for INTER_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inter_clk_div::R](R) reader structure"]
impl crate::Readable for INTER_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inter_clk_div::W](W) writer structure"]
impl crate::Writable for INTER_CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTER_CLK_DIV to value 0"]
impl crate::Resettable for INTER_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
