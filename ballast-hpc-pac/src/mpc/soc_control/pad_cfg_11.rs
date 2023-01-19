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
pub type PAD_33_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_33` writer - "]
pub type PAD_33_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_11_SPEC, u16, u16, 10, O>;
#[doc = "Field `PAD_34` reader - "]
pub type PAD_34_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_34` writer - "]
pub type PAD_34_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_11_SPEC, u16, u16, 10, O>;
#[doc = "Field `PAD_35` reader - "]
pub type PAD_35_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_35` writer - "]
pub type PAD_35_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_11_SPEC, u16, u16, 10, O>;
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
    #[must_use]
    pub fn pad_33(&mut self) -> PAD_33_W<0> {
        PAD_33_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn pad_34(&mut self) -> PAD_34_W<10> {
        PAD_34_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn pad_35(&mut self) -> PAD_35_W<20> {
        PAD_35_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_11 to value 0"]
impl crate::Resettable for PAD_CFG_11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
