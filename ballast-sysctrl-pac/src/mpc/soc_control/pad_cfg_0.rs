#[doc = "Register `PAD_CFG_0` reader"]
pub struct R(crate::R<PAD_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_0` writer"]
pub struct W(crate::W<PAD_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_0_SPEC>;
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
impl From<crate::W<PAD_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_0` reader - "]
pub type PAD_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_0` writer - "]
pub type PAD_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `PAD_1` reader - "]
pub type PAD_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_1` writer - "]
pub type PAD_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `PAD_2` reader - "]
pub type PAD_2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_2` writer - "]
pub type PAD_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_0_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pad_0(&self) -> PAD_0_R {
        PAD_0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn pad_1(&self) -> PAD_1_R {
        PAD_1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn pad_2(&self) -> PAD_2_R {
        PAD_2_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn pad_0(&mut self) -> PAD_0_W<0> {
        PAD_0_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn pad_1(&mut self) -> PAD_1_W<10> {
        PAD_1_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn pad_2(&mut self) -> PAD_2_W<20> {
        PAD_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_0](index.html) module"]
pub struct PAD_CFG_0_SPEC;
impl crate::RegisterSpec for PAD_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_0::R](R) reader structure"]
impl crate::Readable for PAD_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_0::W](W) writer structure"]
impl crate::Writable for PAD_CFG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_0 to value 0"]
impl crate::Resettable for PAD_CFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
