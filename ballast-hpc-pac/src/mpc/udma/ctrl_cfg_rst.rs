#[doc = "Register `CTRL_CFG_RST` reader"]
pub struct R(crate::R<CTRL_CFG_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_CFG_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_CFG_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_CFG_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_CFG_RST` writer"]
pub struct W(crate::W<CTRL_CFG_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_CFG_RST_SPEC>;
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
impl From<crate::W<CTRL_CFG_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_CFG_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL_CFG_RST` reader - uDMA peripherals reset trigger (unimplemented)"]
pub struct CTRL_CFG_RST_R(crate::FieldReader<u32>);
impl CTRL_CFG_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CTRL_CFG_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_CFG_RST_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_CFG_RST` writer - uDMA peripherals reset trigger (unimplemented)"]
pub struct CTRL_CFG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_CFG_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    pub fn ctrl_cfg_rst(&self) -> CTRL_CFG_RST_R {
        CTRL_CFG_RST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    pub fn ctrl_cfg_rst(&mut self) -> CTRL_CFG_RST_W {
        CTRL_CFG_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA peripherals reset trigger (unimplemented)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_cfg_rst](index.html) module"]
pub struct CTRL_CFG_RST_SPEC;
impl crate::RegisterSpec for CTRL_CFG_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_cfg_rst::R](R) reader structure"]
impl crate::Readable for CTRL_CFG_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_cfg_rst::W](W) writer structure"]
impl crate::Writable for CTRL_CFG_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_CFG_RST to value 0"]
impl crate::Resettable for CTRL_CFG_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
