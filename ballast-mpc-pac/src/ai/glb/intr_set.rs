#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDP_DONE_SET0` writer - "]
pub struct SDP_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDP_DONE_SET0_W<'a> {
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
#[doc = "Field `SDP_DONE_SET1` writer - "]
pub struct SDP_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDP_DONE_SET1_W<'a> {
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
#[doc = "Field `CDP_DONE_SET0` writer - "]
pub struct CDP_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDP_DONE_SET0_W<'a> {
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
#[doc = "Field `CDP_DONE_SET1` writer - "]
pub struct CDP_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDP_DONE_SET1_W<'a> {
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
#[doc = "Field `PDP_DONE_SET0` writer - "]
pub struct PDP_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDP_DONE_SET0_W<'a> {
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
#[doc = "Field `PDP_DONE_SET1` writer - "]
pub struct PDP_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDP_DONE_SET1_W<'a> {
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
#[doc = "Field `BDMA_DONE_SET0` writer - "]
pub struct BDMA_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA_DONE_SET0_W<'a> {
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
#[doc = "Field `BDMA_DONE_SET1` writer - "]
pub struct BDMA_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA_DONE_SET1_W<'a> {
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
#[doc = "Field `RUBIK_DONE_SET0` writer - "]
pub struct RUBIK_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> RUBIK_DONE_SET0_W<'a> {
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
#[doc = "Field `RUBIK_DONE_SET1` writer - "]
pub struct RUBIK_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> RUBIK_DONE_SET1_W<'a> {
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
#[doc = "Field `CDMA_DAT_DONE_SET0` writer - "]
pub struct CDMA_DAT_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_DAT_DONE_SET0_W<'a> {
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
#[doc = "Field `CDMA_DAT_DONE_SET1` writer - "]
pub struct CDMA_DAT_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_DAT_DONE_SET1_W<'a> {
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
#[doc = "Field `CDMA_WT_DONE_SET0` writer - "]
pub struct CDMA_WT_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_WT_DONE_SET0_W<'a> {
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
#[doc = "Field `CDMA_WT_DONE_SET1` writer - "]
pub struct CDMA_WT_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMA_WT_DONE_SET1_W<'a> {
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
#[doc = "Field `CACC_DONE_SET0` writer - "]
pub struct CACC_DONE_SET0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACC_DONE_SET0_W<'a> {
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
#[doc = "Field `CACC_DONE_SET1` writer - "]
pub struct CACC_DONE_SET1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACC_DONE_SET1_W<'a> {
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdp_done_set0(&mut self) -> SDP_DONE_SET0_W {
        SDP_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdp_done_set1(&mut self) -> SDP_DONE_SET1_W {
        SDP_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cdp_done_set0(&mut self) -> CDP_DONE_SET0_W {
        CDP_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cdp_done_set1(&mut self) -> CDP_DONE_SET1_W {
        CDP_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pdp_done_set0(&mut self) -> PDP_DONE_SET0_W {
        PDP_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pdp_done_set1(&mut self) -> PDP_DONE_SET1_W {
        PDP_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bdma_done_set0(&mut self) -> BDMA_DONE_SET0_W {
        BDMA_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bdma_done_set1(&mut self) -> BDMA_DONE_SET1_W {
        BDMA_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rubik_done_set0(&mut self) -> RUBIK_DONE_SET0_W {
        RUBIK_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rubik_done_set1(&mut self) -> RUBIK_DONE_SET1_W {
        RUBIK_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cdma_dat_done_set0(&mut self) -> CDMA_DAT_DONE_SET0_W {
        CDMA_DAT_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cdma_dat_done_set1(&mut self) -> CDMA_DAT_DONE_SET1_W {
        CDMA_DAT_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cdma_wt_done_set0(&mut self) -> CDMA_WT_DONE_SET0_W {
        CDMA_WT_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cdma_wt_done_set1(&mut self) -> CDMA_WT_DONE_SET1_W {
        CDMA_WT_DONE_SET1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cacc_done_set0(&mut self) -> CACC_DONE_SET0_W {
        CACC_DONE_SET0_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cacc_done_set1(&mut self) -> CACC_DONE_SET1_W {
        CACC_DONE_SET1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
