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
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDP_DONE_MASK0` reader - "]
pub type SDP_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `SDP_DONE_MASK0` writer - "]
pub type SDP_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `SDP_DONE_MASK1` reader - "]
pub type SDP_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `SDP_DONE_MASK1` writer - "]
pub type SDP_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CDP_DONE_MASK0` reader - "]
pub type CDP_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `CDP_DONE_MASK0` writer - "]
pub type CDP_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CDP_DONE_MASK1` reader - "]
pub type CDP_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `CDP_DONE_MASK1` writer - "]
pub type CDP_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PDP_DONE_MASK0` reader - "]
pub type PDP_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `PDP_DONE_MASK0` writer - "]
pub type PDP_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PDP_DONE_MASK1` reader - "]
pub type PDP_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `PDP_DONE_MASK1` writer - "]
pub type PDP_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `BDMA_DONE_MASK0` reader - "]
pub type BDMA_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `BDMA_DONE_MASK0` writer - "]
pub type BDMA_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `BDMA_DONE_MASK1` reader - "]
pub type BDMA_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `BDMA_DONE_MASK1` writer - "]
pub type BDMA_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RUBIK_DONE_MASK0` reader - "]
pub type RUBIK_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `RUBIK_DONE_MASK0` writer - "]
pub type RUBIK_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RUBIK_DONE_MASK1` reader - "]
pub type RUBIK_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `RUBIK_DONE_MASK1` writer - "]
pub type RUBIK_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CDMA_DAT_DONE_MASK0` reader - "]
pub type CDMA_DAT_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `CDMA_DAT_DONE_MASK0` writer - "]
pub type CDMA_DAT_DONE_MASK0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CDMA_DAT_DONE_MASK1` reader - "]
pub type CDMA_DAT_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `CDMA_DAT_DONE_MASK1` writer - "]
pub type CDMA_DAT_DONE_MASK1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CDMA_WT_DONE_MASK0` reader - "]
pub type CDMA_WT_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `CDMA_WT_DONE_MASK0` writer - "]
pub type CDMA_WT_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CDMA_WT_DONE_MASK1` reader - "]
pub type CDMA_WT_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `CDMA_WT_DONE_MASK1` writer - "]
pub type CDMA_WT_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CACC_DONE_MASK0` reader - "]
pub type CACC_DONE_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `CACC_DONE_MASK0` writer - "]
pub type CACC_DONE_MASK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CACC_DONE_MASK1` reader - "]
pub type CACC_DONE_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `CACC_DONE_MASK1` writer - "]
pub type CACC_DONE_MASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdp_done_mask0(&mut self) -> SDP_DONE_MASK0_W<0> {
        SDP_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdp_done_mask1(&mut self) -> SDP_DONE_MASK1_W<1> {
        SDP_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cdp_done_mask0(&mut self) -> CDP_DONE_MASK0_W<2> {
        CDP_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cdp_done_mask1(&mut self) -> CDP_DONE_MASK1_W<3> {
        CDP_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pdp_done_mask0(&mut self) -> PDP_DONE_MASK0_W<4> {
        PDP_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pdp_done_mask1(&mut self) -> PDP_DONE_MASK1_W<5> {
        PDP_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bdma_done_mask0(&mut self) -> BDMA_DONE_MASK0_W<6> {
        BDMA_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bdma_done_mask1(&mut self) -> BDMA_DONE_MASK1_W<7> {
        BDMA_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rubik_done_mask0(&mut self) -> RUBIK_DONE_MASK0_W<8> {
        RUBIK_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rubik_done_mask1(&mut self) -> RUBIK_DONE_MASK1_W<9> {
        RUBIK_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_dat_done_mask0(&mut self) -> CDMA_DAT_DONE_MASK0_W<16> {
        CDMA_DAT_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_dat_done_mask1(&mut self) -> CDMA_DAT_DONE_MASK1_W<17> {
        CDMA_DAT_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_wt_done_mask0(&mut self) -> CDMA_WT_DONE_MASK0_W<18> {
        CDMA_WT_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_wt_done_mask1(&mut self) -> CDMA_WT_DONE_MASK1_W<19> {
        CDMA_WT_DONE_MASK1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cacc_done_mask0(&mut self) -> CACC_DONE_MASK0_W<20> {
        CACC_DONE_MASK0_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cacc_done_mask1(&mut self) -> CACC_DONE_MASK1_W<21> {
        CACC_DONE_MASK1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
