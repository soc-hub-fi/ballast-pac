#[doc = "Register `CFG_RD_WEIGHT_1` reader"]
pub struct R(crate::R<CFG_RD_WEIGHT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_RD_WEIGHT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_RD_WEIGHT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_RD_WEIGHT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_WEIGHT_SDP_B` reader - "]
pub struct RD_WEIGHT_SDP_B_R(crate::FieldReader<u8>);
impl RD_WEIGHT_SDP_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_SDP_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_SDP_B_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_SDP_N` reader - "]
pub struct RD_WEIGHT_SDP_N_R(crate::FieldReader<u8>);
impl RD_WEIGHT_SDP_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_SDP_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_SDP_N_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_SDP_E` reader - "]
pub struct RD_WEIGHT_SDP_E_R(crate::FieldReader<u8>);
impl RD_WEIGHT_SDP_E_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_SDP_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_SDP_E_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_CDMA_DAT` reader - "]
pub struct RD_WEIGHT_CDMA_DAT_R(crate::FieldReader<u8>);
impl RD_WEIGHT_CDMA_DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_CDMA_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_CDMA_DAT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_weight_sdp_b(&self) -> RD_WEIGHT_SDP_B_R {
        RD_WEIGHT_SDP_B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rd_weight_sdp_n(&self) -> RD_WEIGHT_SDP_N_R {
        RD_WEIGHT_SDP_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rd_weight_sdp_e(&self) -> RD_WEIGHT_SDP_E_R {
        RD_WEIGHT_SDP_E_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_weight_cdma_dat(&self) -> RD_WEIGHT_CDMA_DAT_R {
        RD_WEIGHT_CDMA_DAT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Register1 to control the read weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_rd_weight_1](index.html) module"]
pub struct CFG_RD_WEIGHT_1_SPEC;
impl crate::RegisterSpec for CFG_RD_WEIGHT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_rd_weight_1::R](R) reader structure"]
impl crate::Readable for CFG_RD_WEIGHT_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFG_RD_WEIGHT_1 to value 0x0100_0100"]
impl crate::Resettable for CFG_RD_WEIGHT_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
