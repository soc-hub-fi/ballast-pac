#[doc = "Register `PAD_CFG_14` reader"]
pub struct R(crate::R<PAD_CFG_14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_14` writer"]
pub struct W(crate::W<PAD_CFG_14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_14_SPEC>;
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
impl From<crate::W<PAD_CFG_14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_42` reader - "]
pub type PAD_42_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_42` writer - "]
pub type PAD_42_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_14_SPEC, u16, u16, 10, O>;
#[doc = "Field `PAD_43` reader - "]
pub type PAD_43_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_43` writer - "]
pub type PAD_43_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_14_SPEC, u16, u16, 10, O>;
#[doc = "Field `PAD_44` reader - "]
pub type PAD_44_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_44` writer - "]
pub type PAD_44_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_14_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pad_42(&self) -> PAD_42_R {
        PAD_42_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn pad_43(&self) -> PAD_43_R {
        PAD_43_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn pad_44(&self) -> PAD_44_R {
        PAD_44_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn pad_42(&mut self) -> PAD_42_W<0> {
        PAD_42_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn pad_43(&mut self) -> PAD_43_W<10> {
        PAD_43_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn pad_44(&mut self) -> PAD_44_W<20> {
        PAD_44_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_14](index.html) module"]
pub struct PAD_CFG_14_SPEC;
impl crate::RegisterSpec for PAD_CFG_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_14::R](R) reader structure"]
impl crate::Readable for PAD_CFG_14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_14::W](W) writer structure"]
impl crate::Writable for PAD_CFG_14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_14 to value 0"]
impl crate::Resettable for PAD_CFG_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
