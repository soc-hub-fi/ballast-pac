#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDP_DONE_MASK0` reader - "]
pub struct SDP_DONE_MASK0_R(crate::FieldReader<bool>);
impl SDP_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDP_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDP_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDP_DONE_MASK1` reader - "]
pub struct SDP_DONE_MASK1_R(crate::FieldReader<bool>);
impl SDP_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDP_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDP_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDP_DONE_MASK0` reader - "]
pub struct CDP_DONE_MASK0_R(crate::FieldReader<bool>);
impl CDP_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDP_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDP_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDP_DONE_MASK1` reader - "]
pub struct CDP_DONE_MASK1_R(crate::FieldReader<bool>);
impl CDP_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDP_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDP_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDP_DONE_MASK0` reader - "]
pub struct PDP_DONE_MASK0_R(crate::FieldReader<bool>);
impl PDP_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDP_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDP_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDP_DONE_MASK1` reader - "]
pub struct PDP_DONE_MASK1_R(crate::FieldReader<bool>);
impl PDP_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDP_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDP_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMA_DONE_MASK0` reader - "]
pub struct BDMA_DONE_MASK0_R(crate::FieldReader<bool>);
impl BDMA_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BDMA_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDMA_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMA_DONE_MASK1` reader - "]
pub struct BDMA_DONE_MASK1_R(crate::FieldReader<bool>);
impl BDMA_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BDMA_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDMA_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUBIK_DONE_MASK0` reader - "]
pub struct RUBIK_DONE_MASK0_R(crate::FieldReader<bool>);
impl RUBIK_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUBIK_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUBIK_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUBIK_DONE_MASK1` reader - "]
pub struct RUBIK_DONE_MASK1_R(crate::FieldReader<bool>);
impl RUBIK_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUBIK_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUBIK_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_DAT_DONE_MASK0` reader - "]
pub struct CDMA_DAT_DONE_MASK0_R(crate::FieldReader<bool>);
impl CDMA_DAT_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_DAT_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_DAT_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_DAT_DONE_MASK1` reader - "]
pub struct CDMA_DAT_DONE_MASK1_R(crate::FieldReader<bool>);
impl CDMA_DAT_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_DAT_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_DAT_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_WT_DONE_MASK0` reader - "]
pub struct CDMA_WT_DONE_MASK0_R(crate::FieldReader<bool>);
impl CDMA_WT_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_WT_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_WT_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_WT_DONE_MASK1` reader - "]
pub struct CDMA_WT_DONE_MASK1_R(crate::FieldReader<bool>);
impl CDMA_WT_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_WT_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_WT_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACC_DONE_MASK0` reader - "]
pub struct CACC_DONE_MASK0_R(crate::FieldReader<bool>);
impl CACC_DONE_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACC_DONE_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACC_DONE_MASK0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACC_DONE_MASK1` reader - "]
pub struct CACC_DONE_MASK1_R(crate::FieldReader<bool>);
impl CACC_DONE_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACC_DONE_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACC_DONE_MASK1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdp_done_mask0(&self) -> SDP_DONE_MASK0_R {
        SDP_DONE_MASK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdp_done_mask1(&self) -> SDP_DONE_MASK1_R {
        SDP_DONE_MASK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cdp_done_mask0(&self) -> CDP_DONE_MASK0_R {
        CDP_DONE_MASK0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cdp_done_mask1(&self) -> CDP_DONE_MASK1_R {
        CDP_DONE_MASK1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pdp_done_mask0(&self) -> PDP_DONE_MASK0_R {
        PDP_DONE_MASK0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pdp_done_mask1(&self) -> PDP_DONE_MASK1_R {
        PDP_DONE_MASK1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bdma_done_mask0(&self) -> BDMA_DONE_MASK0_R {
        BDMA_DONE_MASK0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bdma_done_mask1(&self) -> BDMA_DONE_MASK1_R {
        BDMA_DONE_MASK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rubik_done_mask0(&self) -> RUBIK_DONE_MASK0_R {
        RUBIK_DONE_MASK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rubik_done_mask1(&self) -> RUBIK_DONE_MASK1_R {
        RUBIK_DONE_MASK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cdma_dat_done_mask0(&self) -> CDMA_DAT_DONE_MASK0_R {
        CDMA_DAT_DONE_MASK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cdma_dat_done_mask1(&self) -> CDMA_DAT_DONE_MASK1_R {
        CDMA_DAT_DONE_MASK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cdma_wt_done_mask0(&self) -> CDMA_WT_DONE_MASK0_R {
        CDMA_WT_DONE_MASK0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cdma_wt_done_mask1(&self) -> CDMA_WT_DONE_MASK1_R {
        CDMA_WT_DONE_MASK1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cacc_done_mask0(&self) -> CACC_DONE_MASK0_R {
        CACC_DONE_MASK0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cacc_done_mask1(&self) -> CACC_DONE_MASK1_R {
        CACC_DONE_MASK1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Interrupt mask control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
