#[doc = "Register `CFG_RD_WEIGHT_0` reader"]
pub struct R(crate::R<CFG_RD_WEIGHT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_RD_WEIGHT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_RD_WEIGHT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_RD_WEIGHT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_RD_WEIGHT_0` writer"]
pub struct W(crate::W<CFG_RD_WEIGHT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_RD_WEIGHT_0_SPEC>;
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
impl From<crate::W<CFG_RD_WEIGHT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_RD_WEIGHT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_WEIGHT_BDMA` reader - "]
pub struct RD_WEIGHT_BDMA_R(crate::FieldReader<u8, u8>);
impl RD_WEIGHT_BDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_BDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_BDMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_SDP` reader - "]
pub struct RD_WEIGHT_SDP_R(crate::FieldReader<u8, u8>);
impl RD_WEIGHT_SDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_SDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_SDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_PDP` reader - "]
pub struct RD_WEIGHT_PDP_R(crate::FieldReader<u8, u8>);
impl RD_WEIGHT_PDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_PDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_PDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_CDP` reader - "]
pub struct RD_WEIGHT_CDP_R(crate::FieldReader<u8, u8>);
impl RD_WEIGHT_CDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_CDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_CDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_weight_bdma(&self) -> RD_WEIGHT_BDMA_R {
        RD_WEIGHT_BDMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rd_weight_sdp(&self) -> RD_WEIGHT_SDP_R {
        RD_WEIGHT_SDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rd_weight_pdp(&self) -> RD_WEIGHT_PDP_R {
        RD_WEIGHT_PDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_weight_cdp(&self) -> RD_WEIGHT_CDP_R {
        RD_WEIGHT_CDP_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "Register0 to control the read weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_rd_weight_0](index.html) module"]
pub struct CFG_RD_WEIGHT_0_SPEC;
impl crate::RegisterSpec for CFG_RD_WEIGHT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_rd_weight_0::R](R) reader structure"]
impl crate::Readable for CFG_RD_WEIGHT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_rd_weight_0::W](W) writer structure"]
impl crate::Writable for CFG_RD_WEIGHT_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_RD_WEIGHT_0 to value 0x0100_0100"]
impl crate::Resettable for CFG_RD_WEIGHT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
