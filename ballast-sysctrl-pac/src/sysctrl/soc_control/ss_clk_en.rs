#[doc = "Register `SS_CLK_EN` reader"]
pub struct R(crate::R<SS_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SS_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SS_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SS_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SS_CLK_EN` writer"]
pub struct W(crate::W<SS_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SS_CLK_EN_SPEC>;
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
impl From<crate::W<SS_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SS_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pulpissimo` reader - Pulpissimo subsystem clock enable"]
pub type PULPISSIMO_R = crate::BitReader<bool>;
#[doc = "Field `pulpissimo` writer - Pulpissimo subsystem clock enable"]
pub type PULPISSIMO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `interconnect` reader - interconnect subsystem clock enable"]
pub type INTERCONNECT_R = crate::BitReader<bool>;
#[doc = "Field `interconnect` writer - interconnect subsystem clock enable"]
pub type INTERCONNECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `top_peripheral` reader - Top Peripheral subsystem clock enable"]
pub type TOP_PERIPHERAL_R = crate::BitReader<bool>;
#[doc = "Field `top_peripheral` writer - Top Peripheral subsystem clock enable"]
pub type TOP_PERIPHERAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `c2c` reader - C2C clock enable"]
pub type C2C_R = crate::BitReader<bool>;
#[doc = "Field `c2c` writer - C2C clock enable"]
pub type C2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `core_hw` reader - coreHW subsystem clock enable"]
pub type CORE_HW_R = crate::BitReader<bool>;
#[doc = "Field `core_hw` writer - coreHW subsystem clock enable"]
pub type CORE_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `TTA` reader - TTA subsystem clock enable"]
pub type TTA_R = crate::BitReader<bool>;
#[doc = "Field `TTA` writer - TTA subsystem clock enable"]
pub type TTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `ethernet` reader - Ethernet subsystem clock enable"]
pub type ETHERNET_R = crate::BitReader<bool>;
#[doc = "Field `ethernet` writer - Ethernet subsystem clock enable"]
pub type ETHERNET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `AI` reader - AI subsystem clock enable"]
pub type AI_R = crate::BitReader<bool>;
#[doc = "Field `AI` writer - AI subsystem clock enable"]
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
#[doc = "Field `HPC` reader - HPC subsystem clock enable"]
pub type HPC_R = crate::BitReader<bool>;
#[doc = "Field `HPC` writer - HPC subsystem clock enable"]
pub type HPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SS_CLK_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pulpissimo subsystem clock enable"]
    #[inline(always)]
    pub fn pulpissimo(&self) -> PULPISSIMO_R {
        PULPISSIMO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - interconnect subsystem clock enable"]
    #[inline(always)]
    pub fn interconnect(&self) -> INTERCONNECT_R {
        INTERCONNECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Top Peripheral subsystem clock enable"]
    #[inline(always)]
    pub fn top_peripheral(&self) -> TOP_PERIPHERAL_R {
        TOP_PERIPHERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - C2C clock enable"]
    #[inline(always)]
    pub fn c2c(&self) -> C2C_R {
        C2C_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - coreHW subsystem clock enable"]
    #[inline(always)]
    pub fn core_hw(&self) -> CORE_HW_R {
        CORE_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - TTA subsystem clock enable"]
    #[inline(always)]
    pub fn tta(&self) -> TTA_R {
        TTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Ethernet subsystem clock enable"]
    #[inline(always)]
    pub fn ethernet(&self) -> ETHERNET_R {
        ETHERNET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - AI subsystem clock enable"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - HPC subsystem clock enable"]
    #[inline(always)]
    pub fn hpc(&self) -> HPC_R {
        HPC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulpissimo subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pulpissimo(&mut self) -> PULPISSIMO_W<0> {
        PULPISSIMO_W::new(self)
    }
    #[doc = "Bit 4 - interconnect subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn interconnect(&mut self) -> INTERCONNECT_W<4> {
        INTERCONNECT_W::new(self)
    }
    #[doc = "Bit 7 - Top Peripheral subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn top_peripheral(&mut self) -> TOP_PERIPHERAL_W<7> {
        TOP_PERIPHERAL_W::new(self)
    }
    #[doc = "Bit 8 - C2C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2c(&mut self) -> C2C_W<8> {
        C2C_W::new(self)
    }
    #[doc = "Bit 12 - coreHW subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_hw(&mut self) -> CORE_HW_W<12> {
        CORE_HW_W::new(self)
    }
    #[doc = "Bit 16 - TTA subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tta(&mut self) -> TTA_W<16> {
        TTA_W::new(self)
    }
    #[doc = "Bit 20 - Ethernet subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethernet(&mut self) -> ETHERNET_W<20> {
        ETHERNET_W::new(self)
    }
    #[doc = "Bit 24 - AI subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<24> {
        AI_W::new(self)
    }
    #[doc = "Bit 28 - HPC subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpc(&mut self) -> HPC_W<28> {
        HPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss_clk_en](index.html) module"]
pub struct SS_CLK_EN_SPEC;
impl crate::RegisterSpec for SS_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ss_clk_en::R](R) reader structure"]
impl crate::Readable for SS_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ss_clk_en::W](W) writer structure"]
impl crate::Writable for SS_CLK_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SS_CLK_EN to value 0"]
impl crate::Resettable for SS_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
