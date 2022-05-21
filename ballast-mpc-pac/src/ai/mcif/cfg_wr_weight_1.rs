#[doc = "Register `CFG_WR_WEIGHT_1` reader"]
pub struct R(crate::R<CFG_WR_WEIGHT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_WR_WEIGHT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_WR_WEIGHT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_WR_WEIGHT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WR_WEIGHT_RBK` reader - "]
pub struct WR_WEIGHT_RBK_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_RBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_RBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_RBK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_WEIGHT_RSV_2` reader - "]
pub struct WR_WEIGHT_RSV_2_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_RSV_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_RSV_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_RSV_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_WEIGHT_RSV_1` reader - "]
pub struct WR_WEIGHT_RSV_1_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_RSV_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_RSV_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_RSV_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_WEIGHT_RSV_0` reader - "]
pub struct WR_WEIGHT_RSV_0_R(crate::FieldReader<u8, u8>);
impl WR_WEIGHT_RSV_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_WEIGHT_RSV_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_WEIGHT_RSV_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wr_weight_rbk(&self) -> WR_WEIGHT_RBK_R {
        WR_WEIGHT_RBK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wr_weight_rsv_2(&self) -> WR_WEIGHT_RSV_2_R {
        WR_WEIGHT_RSV_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn wr_weight_rsv_1(&self) -> WR_WEIGHT_RSV_1_R {
        WR_WEIGHT_RSV_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wr_weight_rsv_0(&self) -> WR_WEIGHT_RSV_0_R {
        WR_WEIGHT_RSV_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Register1 to control the write weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_wr_weight_1](index.html) module"]
pub struct CFG_WR_WEIGHT_1_SPEC;
impl crate::RegisterSpec for CFG_WR_WEIGHT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_wr_weight_1::R](R) reader structure"]
impl crate::Readable for CFG_WR_WEIGHT_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFG_WR_WEIGHT_1 to value 0x0100_0100"]
impl crate::Resettable for CFG_WR_WEIGHT_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
