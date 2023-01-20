#[doc = "Register `PAD_CFG_15` reader"]
pub struct R(crate::R<PAD_CFG_15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_15` writer"]
pub struct W(crate::W<PAD_CFG_15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_15_SPEC>;
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
impl From<crate::W<PAD_CFG_15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_CFG` reader - "]
pub type PAD_CFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PAD_CFG` writer - "]
pub type PAD_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_15_SPEC, u32, u32, 32, O>;
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
    #[must_use]
    pub fn pad_cfg(&mut self) -> PAD_CFG_W<0> {
        PAD_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_15](index.html) module"]
pub struct PAD_CFG_15_SPEC;
impl crate::RegisterSpec for PAD_CFG_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_15::R](R) reader structure"]
impl crate::Readable for PAD_CFG_15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_15::W](W) writer structure"]
impl crate::Writable for PAD_CFG_15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_15 to value 0"]
impl crate::Resettable for PAD_CFG_15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
