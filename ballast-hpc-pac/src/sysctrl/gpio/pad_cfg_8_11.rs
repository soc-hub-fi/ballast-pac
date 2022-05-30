#[doc = "Register `PAD_CFG_8_11` reader"]
pub struct R(crate::R<PAD_CFG_8_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_8_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_8_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_8_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_8_11` writer"]
pub struct W(crate::W<PAD_CFG_8_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_8_11_SPEC>;
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
impl From<crate::W<PAD_CFG_8_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_8_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_CFG_8` reader - "]
pub struct PAD_CFG_8_R(crate::FieldReader<u8>);
impl PAD_CFG_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_8_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_CFG_8` writer - "]
pub struct PAD_CFG_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PAD_CFG_9` reader - "]
pub struct PAD_CFG_9_R(crate::FieldReader<u8>);
impl PAD_CFG_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_9_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_CFG_9` writer - "]
pub struct PAD_CFG_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PAD_CFG_10` reader - "]
pub struct PAD_CFG_10_R(crate::FieldReader<u8>);
impl PAD_CFG_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_10_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_CFG_10` writer - "]
pub struct PAD_CFG_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PAD_CFG_11` reader - "]
pub struct PAD_CFG_11_R(crate::FieldReader<u8>);
impl PAD_CFG_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_11_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_CFG_11` writer - "]
pub struct PAD_CFG_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_8(&self) -> PAD_CFG_8_R {
        PAD_CFG_8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_9(&self) -> PAD_CFG_9_R {
        PAD_CFG_9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_10(&self) -> PAD_CFG_10_R {
        PAD_CFG_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_11(&self) -> PAD_CFG_11_R {
        PAD_CFG_11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_8(&mut self) -> PAD_CFG_8_W {
        PAD_CFG_8_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_9(&mut self) -> PAD_CFG_9_W {
        PAD_CFG_9_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_10(&mut self) -> PAD_CFG_10_W {
        PAD_CFG_10_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_11(&mut self) -> PAD_CFG_11_W {
        PAD_CFG_11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_8_11](index.html) module"]
pub struct PAD_CFG_8_11_SPEC;
impl crate::RegisterSpec for PAD_CFG_8_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_8_11::R](R) reader structure"]
impl crate::Readable for PAD_CFG_8_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_8_11::W](W) writer structure"]
impl crate::Writable for PAD_CFG_8_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG_8_11 to value 0"]
impl crate::Resettable for PAD_CFG_8_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
