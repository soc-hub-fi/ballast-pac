#[doc = "Register `PAD_CFG_9` reader"]
pub struct R(crate::R<PAD_CFG_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_9` writer"]
pub struct W(crate::W<PAD_CFG_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_9_SPEC>;
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
impl From<crate::W<PAD_CFG_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_cfg_36` reader - "]
pub struct PAD_CFG_36_R(crate::FieldReader<u8>);
impl PAD_CFG_36_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_36_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_36` writer - "]
pub struct PAD_CFG_36_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `pad_cfg_37` reader - "]
pub struct PAD_CFG_37_R(crate::FieldReader<u8>);
impl PAD_CFG_37_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_37_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_37` writer - "]
pub struct PAD_CFG_37_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `pad_cfg_38` reader - "]
pub struct PAD_CFG_38_R(crate::FieldReader<u8>);
impl PAD_CFG_38_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_38_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_38` writer - "]
pub struct PAD_CFG_38_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `pad_cfg_39` reader - "]
pub struct PAD_CFG_39_R(crate::FieldReader<u8>);
impl PAD_CFG_39_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_39_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_39` writer - "]
pub struct PAD_CFG_39_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pad_cfg_36(&self) -> PAD_CFG_36_R {
        PAD_CFG_36_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pad_cfg_37(&self) -> PAD_CFG_37_R {
        PAD_CFG_37_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn pad_cfg_38(&self) -> PAD_CFG_38_R {
        PAD_CFG_38_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn pad_cfg_39(&self) -> PAD_CFG_39_R {
        PAD_CFG_39_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pad_cfg_36(&mut self) -> PAD_CFG_36_W {
        PAD_CFG_36_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pad_cfg_37(&mut self) -> PAD_CFG_37_W {
        PAD_CFG_37_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn pad_cfg_38(&mut self) -> PAD_CFG_38_W {
        PAD_CFG_38_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn pad_cfg_39(&mut self) -> PAD_CFG_39_W {
        PAD_CFG_39_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_9](index.html) module"]
pub struct PAD_CFG_9_SPEC;
impl crate::RegisterSpec for PAD_CFG_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_9::R](R) reader structure"]
impl crate::Readable for PAD_CFG_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_9::W](W) writer structure"]
impl crate::Writable for PAD_CFG_9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG_9 to value 0"]
impl crate::Resettable for PAD_CFG_9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
