#[doc = "Register `PADCFG_00_07` reader"]
pub struct R(crate::R<PADCFG_00_07_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG_00_07_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG_00_07_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG_00_07_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG_00_07` writer"]
pub struct W(crate::W<PADCFG_00_07_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG_00_07_SPEC>;
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
impl From<crate::W<PADCFG_00_07_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG_00_07_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_CFG_0` reader - "]
pub type PAD_CFG_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_CFG_0` writer - "]
pub type PAD_CFG_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCFG_00_07_SPEC, u8, u8, 8, O>;
#[doc = "Field `PAD_CFG_1` reader - "]
pub type PAD_CFG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_CFG_1` writer - "]
pub type PAD_CFG_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCFG_00_07_SPEC, u8, u8, 8, O>;
#[doc = "Field `PAD_CFG_2` reader - "]
pub type PAD_CFG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_CFG_2` writer - "]
pub type PAD_CFG_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCFG_00_07_SPEC, u8, u8, 8, O>;
#[doc = "Field `PAD_CFG_3` reader - "]
pub type PAD_CFG_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_CFG_3` writer - "]
pub type PAD_CFG_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCFG_00_07_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_0(&self) -> PAD_CFG_0_R {
        PAD_CFG_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_1(&self) -> PAD_CFG_1_R {
        PAD_CFG_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_2(&self) -> PAD_CFG_2_R {
        PAD_CFG_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_3(&self) -> PAD_CFG_3_R {
        PAD_CFG_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_0(&mut self) -> PAD_CFG_0_W<0> {
        PAD_CFG_0_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_1(&mut self) -> PAD_CFG_1_W<8> {
        PAD_CFG_1_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_2(&mut self) -> PAD_CFG_2_W<16> {
        PAD_CFG_2_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_3(&mut self) -> PAD_CFG_3_W<24> {
        PAD_CFG_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "supports configuration for 4 PADS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg_00_07](index.html) module"]
pub struct PADCFG_00_07_SPEC;
impl crate::RegisterSpec for PADCFG_00_07_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg_00_07::R](R) reader structure"]
impl crate::Readable for PADCFG_00_07_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg_00_07::W](W) writer structure"]
impl crate::Writable for PADCFG_00_07_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCFG_00_07 to value 0"]
impl crate::Resettable for PADCFG_00_07_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
