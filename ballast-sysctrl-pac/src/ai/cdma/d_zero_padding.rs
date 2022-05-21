#[doc = "Register `D_ZERO_PADDING` reader"]
pub struct R(crate::R<D_ZERO_PADDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ZERO_PADDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ZERO_PADDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ZERO_PADDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_ZERO_PADDING` writer"]
pub struct W(crate::W<D_ZERO_PADDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_ZERO_PADDING_SPEC>;
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
impl From<crate::W<D_ZERO_PADDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_ZERO_PADDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_LEFT` reader - "]
pub struct PAD_LEFT_R(crate::FieldReader<u8, u8>);
impl PAD_LEFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_LEFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_LEFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_LEFT` writer - "]
pub struct PAD_LEFT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_LEFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PAD_RIGHT` reader - "]
pub struct PAD_RIGHT_R(crate::FieldReader<u8, u8>);
impl PAD_RIGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_RIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_RIGHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_RIGHT` writer - "]
pub struct PAD_RIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_RIGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `PAD_TOP` reader - "]
pub struct PAD_TOP_R(crate::FieldReader<u8, u8>);
impl PAD_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_TOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_TOP` writer - "]
pub struct PAD_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `PAD_BOTTOM` reader - "]
pub struct PAD_BOTTOM_R(crate::FieldReader<u8, u8>);
impl PAD_BOTTOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_BOTTOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_BOTTOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_BOTTOM` writer - "]
pub struct PAD_BOTTOM_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_BOTTOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pad_left(&self) -> PAD_LEFT_R {
        PAD_LEFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pad_right(&self) -> PAD_RIGHT_R {
        PAD_RIGHT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn pad_top(&self) -> PAD_TOP_R {
        PAD_TOP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn pad_bottom(&self) -> PAD_BOTTOM_R {
        PAD_BOTTOM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pad_left(&mut self) -> PAD_LEFT_W {
        PAD_LEFT_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pad_right(&mut self) -> PAD_RIGHT_W {
        PAD_RIGHT_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn pad_top(&mut self) -> PAD_TOP_W {
        PAD_TOP_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn pad_bottom(&mut self) -> PAD_BOTTOM_W {
        PAD_BOTTOM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Left/right/top/bottom padding size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_zero_padding](index.html) module"]
pub struct D_ZERO_PADDING_SPEC;
impl crate::RegisterSpec for D_ZERO_PADDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_zero_padding::R](R) reader structure"]
impl crate::Readable for D_ZERO_PADDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_zero_padding::W](W) writer structure"]
impl crate::Writable for D_ZERO_PADDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_ZERO_PADDING to value 0"]
impl crate::Resettable for D_ZERO_PADDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
