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
pub type SDP_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `SDP_DONE_SET1` writer - "]
pub type SDP_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CDP_DONE_SET0` writer - "]
pub type CDP_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CDP_DONE_SET1` writer - "]
pub type CDP_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `PDP_DONE_SET0` writer - "]
pub type PDP_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `PDP_DONE_SET1` writer - "]
pub type PDP_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `BDMA_DONE_SET0` writer - "]
pub type BDMA_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `BDMA_DONE_SET1` writer - "]
pub type BDMA_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RUBIK_DONE_SET0` writer - "]
pub type RUBIK_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RUBIK_DONE_SET1` writer - "]
pub type RUBIK_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CDMA_DAT_DONE_SET0` writer - "]
pub type CDMA_DAT_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CDMA_DAT_DONE_SET1` writer - "]
pub type CDMA_DAT_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CDMA_WT_DONE_SET0` writer - "]
pub type CDMA_WT_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CDMA_WT_DONE_SET1` writer - "]
pub type CDMA_WT_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CACC_DONE_SET0` writer - "]
pub type CACC_DONE_SET0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CACC_DONE_SET1` writer - "]
pub type CACC_DONE_SET1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdp_done_set0(&mut self) -> SDP_DONE_SET0_W<0> {
        SDP_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdp_done_set1(&mut self) -> SDP_DONE_SET1_W<1> {
        SDP_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cdp_done_set0(&mut self) -> CDP_DONE_SET0_W<2> {
        CDP_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cdp_done_set1(&mut self) -> CDP_DONE_SET1_W<3> {
        CDP_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pdp_done_set0(&mut self) -> PDP_DONE_SET0_W<4> {
        PDP_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pdp_done_set1(&mut self) -> PDP_DONE_SET1_W<5> {
        PDP_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bdma_done_set0(&mut self) -> BDMA_DONE_SET0_W<6> {
        BDMA_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bdma_done_set1(&mut self) -> BDMA_DONE_SET1_W<7> {
        BDMA_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rubik_done_set0(&mut self) -> RUBIK_DONE_SET0_W<8> {
        RUBIK_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rubik_done_set1(&mut self) -> RUBIK_DONE_SET1_W<9> {
        RUBIK_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_dat_done_set0(&mut self) -> CDMA_DAT_DONE_SET0_W<16> {
        CDMA_DAT_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_dat_done_set1(&mut self) -> CDMA_DAT_DONE_SET1_W<17> {
        CDMA_DAT_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_wt_done_set0(&mut self) -> CDMA_WT_DONE_SET0_W<18> {
        CDMA_WT_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cdma_wt_done_set1(&mut self) -> CDMA_WT_DONE_SET1_W<19> {
        CDMA_WT_DONE_SET1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cacc_done_set0(&mut self) -> CACC_DONE_SET0_W<20> {
        CACC_DONE_SET0_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cacc_done_set1(&mut self) -> CACC_DONE_SET1_W<21> {
        CACC_DONE_SET1_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
