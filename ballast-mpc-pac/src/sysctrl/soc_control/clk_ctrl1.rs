#[doc = "Register `CLK_CTRL1` reader"]
pub struct R(crate::R<CLK_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL1` writer"]
pub struct W(crate::W<CLK_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL1_SPEC>;
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
impl From<crate::W<CLK_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tta_sel_cka` reader - Select CKA"]
pub type TTA_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `tta_sel_cka` writer - Select CKA"]
pub type TTA_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `tta_force_cka` reader - Force CKA"]
pub type TTA_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `tta_force_cka` writer - Force CKA"]
pub type TTA_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `tta_force_ckb` reader - Force CKB"]
pub type TTA_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `tta_force_ckb` writer - Force CKB"]
pub type TTA_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `tta_subsys_clkena` reader - Subsystem clock enable"]
pub type TTA_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `tta_subsys_clkena` writer - Subsystem clock enable"]
pub type TTA_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `tta_pll_ctrl_valid` reader - PLL Control valid"]
pub type TTA_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `tta_pll_ctrl_valid` writer - PLL Control valid"]
pub type TTA_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ethernet_sel_cka` reader - Select CKA"]
pub type ETHERNET_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `ethernet_sel_cka` writer - Select CKA"]
pub type ETHERNET_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ethernet_force_cka` reader - Force CKA"]
pub type ETHERNET_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `ethernet_force_cka` writer - Force CKA"]
pub type ETHERNET_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ethernet_force_ckb` reader - Force CKB"]
pub type ETHERNET_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `ethernet_force_ckb` writer - Force CKB"]
pub type ETHERNET_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ethernet_subsys_clkena` reader - Subsystem clock enable"]
pub type ETHERNET_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `ethernet_subsys_clkena` writer - Subsystem clock enable"]
pub type ETHERNET_SUBSYS_CLKENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ethernet_pll_ctrl_valid` reader - PLL Control valid"]
pub type ETHERNET_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `ethernet_pll_ctrl_valid` writer - PLL Control valid"]
pub type ETHERNET_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `AI_sel_cka` reader - Select CKA"]
pub type AI_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `AI_sel_cka` writer - Select CKA"]
pub type AI_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `AI_force_cka` reader - Force CKA"]
pub type AI_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `AI_force_cka` writer - Force CKA"]
pub type AI_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `AI_force_ckb` reader - Force CKB"]
pub type AI_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `AI_force_ckb` writer - Force CKB"]
pub type AI_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `AI_subsys_clkena` reader - Subsystem clock enable"]
pub type AI_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `AI_subsys_clkena` writer - Subsystem clock enable"]
pub type AI_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `AI_pll_ctrl_valid` reader - PLL Control valid"]
pub type AI_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `AI_pll_ctrl_valid` writer - PLL Control valid"]
pub type AI_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `hpc_sel_cka` reader - Select CKA"]
pub type HPC_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `hpc_sel_cka` writer - Select CKA"]
pub type HPC_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `hpc_force_cka` reader - Force CKA"]
pub type HPC_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `hpc_force_cka` writer - Force CKA"]
pub type HPC_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `hpc_force_ckb` reader - Force CKB"]
pub type HPC_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `hpc_force_ckb` writer - Force CKB"]
pub type HPC_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `hpc_subsys_clkena` reader - Subsystem clock enable"]
pub type HPC_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `hpc_subsys_clkena` writer - Subsystem clock enable"]
pub type HPC_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `hpc_pll_ctrl_valid` reader - PLL Control valid"]
pub type HPC_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `hpc_pll_ctrl_valid` writer - PLL Control valid"]
pub type HPC_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn tta_sel_cka(&self) -> TTA_SEL_CKA_R {
        TTA_SEL_CKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn tta_force_cka(&self) -> TTA_FORCE_CKA_R {
        TTA_FORCE_CKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn tta_force_ckb(&self) -> TTA_FORCE_CKB_R {
        TTA_FORCE_CKB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn tta_subsys_clkena(&self) -> TTA_SUBSYS_CLKENA_R {
        TTA_SUBSYS_CLKENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn tta_pll_ctrl_valid(&self) -> TTA_PLL_CTRL_VALID_R {
        TTA_PLL_CTRL_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    pub fn ethernet_sel_cka(&self) -> ETHERNET_SEL_CKA_R {
        ETHERNET_SEL_CKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    pub fn ethernet_force_cka(&self) -> ETHERNET_FORCE_CKA_R {
        ETHERNET_FORCE_CKA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    pub fn ethernet_force_ckb(&self) -> ETHERNET_FORCE_CKB_R {
        ETHERNET_FORCE_CKB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    pub fn ethernet_subsys_clkena(&self) -> ETHERNET_SUBSYS_CLKENA_R {
        ETHERNET_SUBSYS_CLKENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    pub fn ethernet_pll_ctrl_valid(&self) -> ETHERNET_PLL_CTRL_VALID_R {
        ETHERNET_PLL_CTRL_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    pub fn ai_sel_cka(&self) -> AI_SEL_CKA_R {
        AI_SEL_CKA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    pub fn ai_force_cka(&self) -> AI_FORCE_CKA_R {
        AI_FORCE_CKA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    pub fn ai_force_ckb(&self) -> AI_FORCE_CKB_R {
        AI_FORCE_CKB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    pub fn ai_subsys_clkena(&self) -> AI_SUBSYS_CLKENA_R {
        AI_SUBSYS_CLKENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    pub fn ai_pll_ctrl_valid(&self) -> AI_PLL_CTRL_VALID_R {
        AI_PLL_CTRL_VALID_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    pub fn hpc_sel_cka(&self) -> HPC_SEL_CKA_R {
        HPC_SEL_CKA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    pub fn hpc_force_cka(&self) -> HPC_FORCE_CKA_R {
        HPC_FORCE_CKA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    pub fn hpc_force_ckb(&self) -> HPC_FORCE_CKB_R {
        HPC_FORCE_CKB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    pub fn hpc_subsys_clkena(&self) -> HPC_SUBSYS_CLKENA_R {
        HPC_SUBSYS_CLKENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    pub fn hpc_pll_ctrl_valid(&self) -> HPC_PLL_CTRL_VALID_R {
        HPC_PLL_CTRL_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn tta_sel_cka(&mut self) -> TTA_SEL_CKA_W<0> {
        TTA_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn tta_force_cka(&mut self) -> TTA_FORCE_CKA_W<1> {
        TTA_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn tta_force_ckb(&mut self) -> TTA_FORCE_CKB_W<2> {
        TTA_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tta_subsys_clkena(&mut self) -> TTA_SUBSYS_CLKENA_W<3> {
        TTA_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn tta_pll_ctrl_valid(&mut self) -> TTA_PLL_CTRL_VALID_W<7> {
        TTA_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn ethernet_sel_cka(&mut self) -> ETHERNET_SEL_CKA_W<8> {
        ETHERNET_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn ethernet_force_cka(&mut self) -> ETHERNET_FORCE_CKA_W<9> {
        ETHERNET_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn ethernet_force_ckb(&mut self) -> ETHERNET_FORCE_CKB_W<10> {
        ETHERNET_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethernet_subsys_clkena(&mut self) -> ETHERNET_SUBSYS_CLKENA_W<11> {
        ETHERNET_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn ethernet_pll_ctrl_valid(&mut self) -> ETHERNET_PLL_CTRL_VALID_W<15> {
        ETHERNET_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn ai_sel_cka(&mut self) -> AI_SEL_CKA_W<16> {
        AI_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn ai_force_cka(&mut self) -> AI_FORCE_CKA_W<17> {
        AI_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn ai_force_ckb(&mut self) -> AI_FORCE_CKB_W<18> {
        AI_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ai_subsys_clkena(&mut self) -> AI_SUBSYS_CLKENA_W<19> {
        AI_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn ai_pll_ctrl_valid(&mut self) -> AI_PLL_CTRL_VALID_W<23> {
        AI_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_sel_cka(&mut self) -> HPC_SEL_CKA_W<24> {
        HPC_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_force_cka(&mut self) -> HPC_FORCE_CKA_W<25> {
        HPC_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_force_ckb(&mut self) -> HPC_FORCE_CKB_W<26> {
        HPC_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_subsys_clkena(&mut self) -> HPC_SUBSYS_CLKENA_W<27> {
        HPC_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_pll_ctrl_valid(&mut self) -> HPC_PLL_CTRL_VALID_W<31> {
        HPC_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition for TTA, Ethernet, AI, HPC subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl1](index.html) module"]
pub struct CLK_CTRL1_SPEC;
impl crate::RegisterSpec for CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl1::R](R) reader structure"]
impl crate::Readable for CLK_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl1::W](W) writer structure"]
impl crate::Writable for CLK_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CTRL1 to value 0"]
impl crate::Resettable for CLK_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
