#[doc = "Register `PAD_CFG_5` reader"]
pub struct R(crate::R<PAD_CFG_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_5` writer"]
pub struct W(crate::W<PAD_CFG_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_5_SPEC>;
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
impl From<crate::W<PAD_CFG_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_cfg_20` reader - "]
pub struct PAD_CFG_20_R(crate::FieldReader<u8>);
impl PAD_CFG_20_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_20_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_20` writer - "]
pub struct PAD_CFG_20_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `pad_cfg_21` reader - "]
pub struct PAD_CFG_21_R(crate::FieldReader<u8>);
impl PAD_CFG_21_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_21_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_21` writer - "]
pub struct PAD_CFG_21_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `pad_cfg_22` reader - "]
pub struct PAD_CFG_22_R(crate::FieldReader<u8>);
impl PAD_CFG_22_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_22_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_22` writer - "]
pub struct PAD_CFG_22_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `pad_cfg_23` reader - "]
pub struct PAD_CFG_23_R(crate::FieldReader<u8>);
impl PAD_CFG_23_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_CFG_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_23_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_cfg_23` writer - "]
pub struct PAD_CFG_23_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_23_W<'a> {
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
    pub fn pad_cfg_20(&self) -> PAD_CFG_20_R {
        PAD_CFG_20_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pad_cfg_21(&self) -> PAD_CFG_21_R {
        PAD_CFG_21_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn pad_cfg_22(&self) -> PAD_CFG_22_R {
        PAD_CFG_22_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn pad_cfg_23(&self) -> PAD_CFG_23_R {
        PAD_CFG_23_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pad_cfg_20(&mut self) -> PAD_CFG_20_W {
        PAD_CFG_20_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pad_cfg_21(&mut self) -> PAD_CFG_21_W {
        PAD_CFG_21_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn pad_cfg_22(&mut self) -> PAD_CFG_22_W {
        PAD_CFG_22_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn pad_cfg_23(&mut self) -> PAD_CFG_23_W {
        PAD_CFG_23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_5](index.html) module"]
pub struct PAD_CFG_5_SPEC;
impl crate::RegisterSpec for PAD_CFG_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_5::R](R) reader structure"]
impl crate::Readable for PAD_CFG_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_5::W](W) writer structure"]
impl crate::Writable for PAD_CFG_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG_5 to value 0"]
impl crate::Resettable for PAD_CFG_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
