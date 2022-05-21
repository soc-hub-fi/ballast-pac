#[doc = "Register `PAD_CFG_11` reader"]
pub struct R(crate::R<PAD_CFG_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_11` writer"]
pub struct W(crate::W<PAD_CFG_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_11_SPEC>;
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
impl From<crate::W<PAD_CFG_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_33` reader - "]
pub struct PAD_33_R(crate::FieldReader<u16, u16>);
impl PAD_33_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAD_33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_33_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_33` writer - "]
pub struct PAD_33_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `PAD_34` reader - "]
pub struct PAD_34_R(crate::FieldReader<u16, u16>);
impl PAD_34_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAD_34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_34_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_34` writer - "]
pub struct PAD_34_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `PAD_35` reader - "]
pub struct PAD_35_R(crate::FieldReader<u16, u16>);
impl PAD_35_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAD_35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_35_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_35` writer - "]
pub struct PAD_35_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pad_33(&self) -> PAD_33_R {
        PAD_33_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn pad_34(&self) -> PAD_34_R {
        PAD_34_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn pad_35(&self) -> PAD_35_R {
        PAD_35_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pad_33(&mut self) -> PAD_33_W {
        PAD_33_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn pad_34(&mut self) -> PAD_34_W {
        PAD_34_W { w: self }
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn pad_35(&mut self) -> PAD_35_W {
        PAD_35_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_11](index.html) module"]
pub struct PAD_CFG_11_SPEC;
impl crate::RegisterSpec for PAD_CFG_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_11::R](R) reader structure"]
impl crate::Readable for PAD_CFG_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_11::W](W) writer structure"]
impl crate::Writable for PAD_CFG_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG_11 to value 0"]
impl crate::Resettable for PAD_CFG_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
