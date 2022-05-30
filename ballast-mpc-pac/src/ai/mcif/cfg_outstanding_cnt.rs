#[doc = "Register `CFG_OUTSTANDING_CNT` reader"]
pub struct R(crate::R<CFG_OUTSTANDING_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_OUTSTANDING_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_OUTSTANDING_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_OUTSTANDING_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_OS_CNT` reader - "]
pub struct RD_OS_CNT_R(crate::FieldReader<u8>);
impl RD_OS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_OS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_OS_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_OS_CNT` reader - "]
pub struct WR_OS_CNT_R(crate::FieldReader<u8>);
impl WR_OS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WR_OS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_OS_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_os_cnt(&self) -> RD_OS_CNT_R {
        RD_OS_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wr_os_cnt(&self) -> WR_OS_CNT_R {
        WR_OS_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Outstanding AXI transactions in unit of 64Byte\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_outstanding_cnt](index.html) module"]
pub struct CFG_OUTSTANDING_CNT_SPEC;
impl crate::RegisterSpec for CFG_OUTSTANDING_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_outstanding_cnt::R](R) reader structure"]
impl crate::Readable for CFG_OUTSTANDING_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFG_OUTSTANDING_CNT to value 0x00ff_00ff"]
impl crate::Resettable for CFG_OUTSTANDING_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_00ff
    }
}
