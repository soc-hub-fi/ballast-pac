#[doc = "Register `CFG_WR_WEIGHT_0` reader"]
pub struct R(crate::R<CFG_WR_WEIGHT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_WR_WEIGHT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_WR_WEIGHT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_WR_WEIGHT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_WR_WEIGHT_0` writer"]
pub struct W(crate::W<CFG_WR_WEIGHT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_WR_WEIGHT_0_SPEC>;
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
impl From<crate::W<CFG_WR_WEIGHT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_WR_WEIGHT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_WEIGHT_BDMA` reader - "]
pub struct WR_WEIGHT_BDMA_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_BDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_BDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_BDMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_WEIGHT_SDP` reader - "]
pub struct WR_WEIGHT_SDP_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_SDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_SDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_SDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_WEIGHT_PDP` reader - "]
pub struct WR_WEIGHT_PDP_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_PDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_PDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_PDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_WEIGHT_CDP` reader - "]
pub struct WR_WEIGHT_CDP_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_CDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_CDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_CDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wr_weight_bdma(&self) -> WR_WEIGHT_BDMA_R {
        WR_WEIGHT_BDMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wr_weight_sdp(&self) -> WR_WEIGHT_SDP_R {
        WR_WEIGHT_SDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn wr_weight_pdp(&self) -> WR_WEIGHT_PDP_R {
        WR_WEIGHT_PDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wr_weight_cdp(&self) -> WR_WEIGHT_CDP_R {
        WR_WEIGHT_CDP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register0 to control the write weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_wr_weight_0](index.html) module"]
pub struct CFG_WR_WEIGHT_0_SPEC;
impl crate::RegisterSpec for CFG_WR_WEIGHT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_wr_weight_0::R](R) reader structure"]
impl crate::Readable for CFG_WR_WEIGHT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_wr_weight_0::W](W) writer structure"]
impl crate::Writable for CFG_WR_WEIGHT_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_WR_WEIGHT_0 to value 0x0100_0100"]
impl crate::Resettable for CFG_WR_WEIGHT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
