#[doc = "Register `PAD_CFG_6` reader"]
pub struct R(crate::R<PAD_CFG_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_6` writer"]
pub struct W(crate::W<PAD_CFG_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_6_SPEC>;
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
impl From<crate::W<PAD_CFG_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_CFG` reader - "]
pub struct PAD_CFG_R(crate::FieldReader<u32, u32>);
impl PAD_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PAD_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_CFG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_CFG` writer - "]
pub struct PAD_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pad_cfg(&self) -> PAD_CFG_R {
        PAD_CFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pad_cfg(&mut self) -> PAD_CFG_W {
        PAD_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_6](index.html) module"]
pub struct PAD_CFG_6_SPEC;
impl crate::RegisterSpec for PAD_CFG_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_6::R](R) reader structure"]
impl crate::Readable for PAD_CFG_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_6::W](W) writer structure"]
impl crate::Writable for PAD_CFG_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG_6 to value 0"]
impl crate::Resettable for PAD_CFG_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
