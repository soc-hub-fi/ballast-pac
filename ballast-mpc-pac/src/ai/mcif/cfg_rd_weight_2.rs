#[doc = "Register `CFG_RD_WEIGHT_2` reader"]
pub struct R(crate::R<CFG_RD_WEIGHT_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_RD_WEIGHT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_RD_WEIGHT_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_RD_WEIGHT_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_WEIGHT_CDMA_WT` reader - "]
pub struct RD_WEIGHT_CDMA_WT_R(crate::FieldReader<u8>);
impl RD_WEIGHT_CDMA_WT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_CDMA_WT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_CDMA_WT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_RBK` reader - "]
pub struct RD_WEIGHT_RBK_R(crate::FieldReader<u8>);
impl RD_WEIGHT_RBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_RBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_RBK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_RSV_1` reader - "]
pub struct RD_WEIGHT_RSV_1_R(crate::FieldReader<u8>);
impl RD_WEIGHT_RSV_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_RSV_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_RSV_1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WEIGHT_RSV_0` reader - "]
pub struct RD_WEIGHT_RSV_0_R(crate::FieldReader<u8>);
impl RD_WEIGHT_RSV_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_WEIGHT_RSV_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WEIGHT_RSV_0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_weight_cdma_wt(&self) -> RD_WEIGHT_CDMA_WT_R {
        RD_WEIGHT_CDMA_WT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rd_weight_rbk(&self) -> RD_WEIGHT_RBK_R {
        RD_WEIGHT_RBK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rd_weight_rsv_1(&self) -> RD_WEIGHT_RSV_1_R {
        RD_WEIGHT_RSV_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_weight_rsv_0(&self) -> RD_WEIGHT_RSV_0_R {
        RD_WEIGHT_RSV_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Register2 to control the read weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_rd_weight_2](index.html) module"]
pub struct CFG_RD_WEIGHT_2_SPEC;
impl crate::RegisterSpec for CFG_RD_WEIGHT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_rd_weight_2::R](R) reader structure"]
impl crate::Readable for CFG_RD_WEIGHT_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFG_RD_WEIGHT_2 to value 0x0100_0100"]
impl crate::Resettable for CFG_RD_WEIGHT_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
