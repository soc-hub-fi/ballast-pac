#[doc = "Register `INTR_STATUS` reader"]
pub struct R(crate::R<INTR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_STATUS` writer"]
pub struct W(crate::W<INTR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_STATUS_SPEC>;
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
impl From<crate::W<INTR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDP_DONE_STATUS0` reader - "]
pub struct SDP_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl SDP_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDP_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDP_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDP_DONE_STATUS0` writer - "]
pub struct SDP_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDP_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `SDP_DONE_STATUS1` reader - "]
pub struct SDP_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl SDP_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDP_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDP_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDP_DONE_STATUS1` writer - "]
pub struct SDP_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDP_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `CDP_DONE_STATUS0` reader - "]
pub struct CDP_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl CDP_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDP_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDP_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDP_DONE_STATUS0` writer - "]
pub struct CDP_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDP_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `CDP_DONE_STATUS1` reader - "]
pub struct CDP_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl CDP_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDP_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDP_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDP_DONE_STATUS1` writer - "]
pub struct CDP_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDP_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `PDP_DONE_STATUS0` reader - "]
pub struct PDP_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl PDP_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDP_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDP_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDP_DONE_STATUS0` writer - "]
pub struct PDP_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDP_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `PDP_DONE_STATUS1` reader - "]
pub struct PDP_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl PDP_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDP_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDP_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDP_DONE_STATUS1` writer - "]
pub struct PDP_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDP_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `BDMA_DONE_STATUS0` reader - "]
pub struct BDMA_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl BDMA_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BDMA_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDMA_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMA_DONE_STATUS0` writer - "]
pub struct BDMA_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `BDMA_DONE_STATUS1` reader - "]
pub struct BDMA_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl BDMA_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BDMA_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDMA_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDMA_DONE_STATUS1` writer - "]
pub struct BDMA_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RUBIK_DONE_STATUS0` reader - "]
pub struct RUBIK_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl RUBIK_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUBIK_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUBIK_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUBIK_DONE_STATUS0` writer - "]
pub struct RUBIK_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> RUBIK_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RUBIK_DONE_STATUS1` reader - "]
pub struct RUBIK_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl RUBIK_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUBIK_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUBIK_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUBIK_DONE_STATUS1` writer - "]
pub struct RUBIK_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RUBIK_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `CDMA_DAT_DONE_STATUS0` reader - "]
pub struct CDMA_DAT_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl CDMA_DAT_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_DAT_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_DAT_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_DAT_DONE_STATUS0` writer - "]
pub struct CDMA_DAT_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_DAT_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `CDMA_DAT_DONE_STATUS1` reader - "]
pub struct CDMA_DAT_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl CDMA_DAT_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_DAT_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_DAT_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_DAT_DONE_STATUS1` writer - "]
pub struct CDMA_DAT_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_DAT_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `CDMA_WT_DONE_STATUS0` reader - "]
pub struct CDMA_WT_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl CDMA_WT_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_WT_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_WT_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_WT_DONE_STATUS0` writer - "]
pub struct CDMA_WT_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_WT_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `CDMA_WT_DONE_STATUS1` reader - "]
pub struct CDMA_WT_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl CDMA_WT_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDMA_WT_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMA_WT_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMA_WT_DONE_STATUS1` writer - "]
pub struct CDMA_WT_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_WT_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `CACC_DONE_STATUS0` reader - "]
pub struct CACC_DONE_STATUS0_R(crate::FieldReader<bool, bool>);
impl CACC_DONE_STATUS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACC_DONE_STATUS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACC_DONE_STATUS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACC_DONE_STATUS0` writer - "]
pub struct CACC_DONE_STATUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACC_DONE_STATUS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `CACC_DONE_STATUS1` reader - "]
pub struct CACC_DONE_STATUS1_R(crate::FieldReader<bool, bool>);
impl CACC_DONE_STATUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACC_DONE_STATUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACC_DONE_STATUS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACC_DONE_STATUS1` writer - "]
pub struct CACC_DONE_STATUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACC_DONE_STATUS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdp_done_status0(&self) -> SDP_DONE_STATUS0_R {
        SDP_DONE_STATUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdp_done_status1(&self) -> SDP_DONE_STATUS1_R {
        SDP_DONE_STATUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cdp_done_status0(&self) -> CDP_DONE_STATUS0_R {
        CDP_DONE_STATUS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cdp_done_status1(&self) -> CDP_DONE_STATUS1_R {
        CDP_DONE_STATUS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pdp_done_status0(&self) -> PDP_DONE_STATUS0_R {
        PDP_DONE_STATUS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pdp_done_status1(&self) -> PDP_DONE_STATUS1_R {
        PDP_DONE_STATUS1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bdma_done_status0(&self) -> BDMA_DONE_STATUS0_R {
        BDMA_DONE_STATUS0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bdma_done_status1(&self) -> BDMA_DONE_STATUS1_R {
        BDMA_DONE_STATUS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rubik_done_status0(&self) -> RUBIK_DONE_STATUS0_R {
        RUBIK_DONE_STATUS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rubik_done_status1(&self) -> RUBIK_DONE_STATUS1_R {
        RUBIK_DONE_STATUS1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cdma_dat_done_status0(&self) -> CDMA_DAT_DONE_STATUS0_R {
        CDMA_DAT_DONE_STATUS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cdma_dat_done_status1(&self) -> CDMA_DAT_DONE_STATUS1_R {
        CDMA_DAT_DONE_STATUS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cdma_wt_done_status0(&self) -> CDMA_WT_DONE_STATUS0_R {
        CDMA_WT_DONE_STATUS0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cdma_wt_done_status1(&self) -> CDMA_WT_DONE_STATUS1_R {
        CDMA_WT_DONE_STATUS1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cacc_done_status0(&self) -> CACC_DONE_STATUS0_R {
        CACC_DONE_STATUS0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cacc_done_status1(&self) -> CACC_DONE_STATUS1_R {
        CACC_DONE_STATUS1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdp_done_status0(&mut self) -> SDP_DONE_STATUS0_W {
        SDP_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdp_done_status1(&mut self) -> SDP_DONE_STATUS1_W {
        SDP_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cdp_done_status0(&mut self) -> CDP_DONE_STATUS0_W {
        CDP_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cdp_done_status1(&mut self) -> CDP_DONE_STATUS1_W {
        CDP_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pdp_done_status0(&mut self) -> PDP_DONE_STATUS0_W {
        PDP_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pdp_done_status1(&mut self) -> PDP_DONE_STATUS1_W {
        PDP_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bdma_done_status0(&mut self) -> BDMA_DONE_STATUS0_W {
        BDMA_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bdma_done_status1(&mut self) -> BDMA_DONE_STATUS1_W {
        BDMA_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rubik_done_status0(&mut self) -> RUBIK_DONE_STATUS0_W {
        RUBIK_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rubik_done_status1(&mut self) -> RUBIK_DONE_STATUS1_W {
        RUBIK_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cdma_dat_done_status0(&mut self) -> CDMA_DAT_DONE_STATUS0_W {
        CDMA_DAT_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cdma_dat_done_status1(&mut self) -> CDMA_DAT_DONE_STATUS1_W {
        CDMA_DAT_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cdma_wt_done_status0(&mut self) -> CDMA_WT_DONE_STATUS0_W {
        CDMA_WT_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cdma_wt_done_status1(&mut self) -> CDMA_WT_DONE_STATUS1_W {
        CDMA_WT_DONE_STATUS1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cacc_done_status0(&mut self) -> CACC_DONE_STATUS0_W {
        CACC_DONE_STATUS0_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cacc_done_status1(&mut self) -> CACC_DONE_STATUS1_W {
        CACC_DONE_STATUS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_status](index.html) module"]
pub struct INTR_STATUS_SPEC;
impl crate::RegisterSpec for INTR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_status::R](R) reader structure"]
impl crate::Readable for INTR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_status::W](W) writer structure"]
impl crate::Writable for INTR_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_STATUS to value 0"]
impl crate::Resettable for INTR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
