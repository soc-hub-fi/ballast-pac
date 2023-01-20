#[doc = "Register `CLK_CTRL2` reader"]
pub struct R(crate::R<CLK_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL2` writer"]
pub struct W(crate::W<CLK_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL2_SPEC>;
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
impl From<crate::W<CLK_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mpc_sel_cka` reader - Select CKA"]
pub type MPC_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `mpc_sel_cka` writer - Select CKA"]
pub type MPC_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `mpc_force_cka` reader - Force CKA"]
pub type MPC_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `mpc_force_cka` writer - Force CKA"]
pub type MPC_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `mpc_force_ckb` reader - Force CKB"]
pub type MPC_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `mpc_force_ckb` writer - Force CKB"]
pub type MPC_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `mpc_subsys_clkena` reader - Subsystem clock enable"]
pub type MPC_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `mpc_subsys_clkena` writer - Subsystem clock enable"]
pub type MPC_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `mpc_pll_ctrl_valid` reader - PLL Control valid"]
pub type MPC_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `mpc_pll_ctrl_valid` writer - PLL Control valid"]
pub type MPC_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `interconnect_sel_cka` reader - Select CKA"]
pub type INTERCONNECT_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `interconnect_sel_cka` writer - Select CKA"]
pub type INTERCONNECT_SEL_CKA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `interconnect_force_cka` reader - Force CKA"]
pub type INTERCONNECT_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `interconnect_force_cka` writer - Force CKA"]
pub type INTERCONNECT_FORCE_CKA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `interconnect_force_ckb` reader - Force CKB"]
pub type INTERCONNECT_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `interconnect_force_ckb` writer - Force CKB"]
pub type INTERCONNECT_FORCE_CKB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `interconnect_subsys_clkena` reader - Subsystem clock enable"]
pub type INTERCONNECT_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `interconnect_subsys_clkena` writer - Subsystem clock enable"]
pub type INTERCONNECT_SUBSYS_CLKENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `interconnect_pll_ctrl_valid` reader - PLL Control valid"]
pub type INTERCONNECT_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `interconnect_pll_ctrl_valid` writer - PLL Control valid"]
pub type INTERCONNECT_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `c2c_sel_cka` reader - Select CKA"]
pub type C2C_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `c2c_sel_cka` writer - Select CKA"]
pub type C2C_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `c2c_force_cka` reader - Force CKA"]
pub type C2C_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `c2c_force_cka` writer - Force CKA"]
pub type C2C_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `c2c_force_ckb` reader - Force CKB"]
pub type C2C_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `c2c_force_ckb` writer - Force CKB"]
pub type C2C_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `c2c_subsys_clkena` reader - Subsystem clock enable"]
pub type C2C_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `c2c_subsys_clkena` writer - Subsystem clock enable"]
pub type C2C_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `c2c_pll_ctrl_valid` reader - PLL Control valid"]
pub type C2C_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `c2c_pll_ctrl_valid` writer - PLL Control valid"]
pub type C2C_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `corehw_sel_cka` reader - Select CKA"]
pub type COREHW_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `corehw_sel_cka` writer - Select CKA"]
pub type COREHW_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `corehw_force_cka` reader - Force CKA"]
pub type COREHW_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `corehw_force_cka` writer - Force CKA"]
pub type COREHW_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `corehw_force_ckb` reader - Force CKB"]
pub type COREHW_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `corehw_force_ckb` writer - Force CKB"]
pub type COREHW_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `corehw_subsys_clkena` reader - Subsystem clock enable"]
pub type COREHW_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `corehw_subsys_clkena` writer - Subsystem clock enable"]
pub type COREHW_SUBSYS_CLKENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `corehw_pll_ctrl_valid` reader - PLL Control valid"]
pub type COREHW_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `corehw_pll_ctrl_valid` writer - PLL Control valid"]
pub type COREHW_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn mpc_sel_cka(&self) -> MPC_SEL_CKA_R {
        MPC_SEL_CKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn mpc_force_cka(&self) -> MPC_FORCE_CKA_R {
        MPC_FORCE_CKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn mpc_force_ckb(&self) -> MPC_FORCE_CKB_R {
        MPC_FORCE_CKB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn mpc_subsys_clkena(&self) -> MPC_SUBSYS_CLKENA_R {
        MPC_SUBSYS_CLKENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn mpc_pll_ctrl_valid(&self) -> MPC_PLL_CTRL_VALID_R {
        MPC_PLL_CTRL_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    pub fn interconnect_sel_cka(&self) -> INTERCONNECT_SEL_CKA_R {
        INTERCONNECT_SEL_CKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    pub fn interconnect_force_cka(&self) -> INTERCONNECT_FORCE_CKA_R {
        INTERCONNECT_FORCE_CKA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    pub fn interconnect_force_ckb(&self) -> INTERCONNECT_FORCE_CKB_R {
        INTERCONNECT_FORCE_CKB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    pub fn interconnect_subsys_clkena(&self) -> INTERCONNECT_SUBSYS_CLKENA_R {
        INTERCONNECT_SUBSYS_CLKENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    pub fn interconnect_pll_ctrl_valid(&self) -> INTERCONNECT_PLL_CTRL_VALID_R {
        INTERCONNECT_PLL_CTRL_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    pub fn c2c_sel_cka(&self) -> C2C_SEL_CKA_R {
        C2C_SEL_CKA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    pub fn c2c_force_cka(&self) -> C2C_FORCE_CKA_R {
        C2C_FORCE_CKA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    pub fn c2c_force_ckb(&self) -> C2C_FORCE_CKB_R {
        C2C_FORCE_CKB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    pub fn c2c_subsys_clkena(&self) -> C2C_SUBSYS_CLKENA_R {
        C2C_SUBSYS_CLKENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    pub fn c2c_pll_ctrl_valid(&self) -> C2C_PLL_CTRL_VALID_R {
        C2C_PLL_CTRL_VALID_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    pub fn corehw_sel_cka(&self) -> COREHW_SEL_CKA_R {
        COREHW_SEL_CKA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    pub fn corehw_force_cka(&self) -> COREHW_FORCE_CKA_R {
        COREHW_FORCE_CKA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    pub fn corehw_force_ckb(&self) -> COREHW_FORCE_CKB_R {
        COREHW_FORCE_CKB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    pub fn corehw_subsys_clkena(&self) -> COREHW_SUBSYS_CLKENA_R {
        COREHW_SUBSYS_CLKENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    pub fn corehw_pll_ctrl_valid(&self) -> COREHW_PLL_CTRL_VALID_R {
        COREHW_PLL_CTRL_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn mpc_sel_cka(&mut self) -> MPC_SEL_CKA_W<0> {
        MPC_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn mpc_force_cka(&mut self) -> MPC_FORCE_CKA_W<1> {
        MPC_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn mpc_force_ckb(&mut self) -> MPC_FORCE_CKB_W<2> {
        MPC_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpc_subsys_clkena(&mut self) -> MPC_SUBSYS_CLKENA_W<3> {
        MPC_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn mpc_pll_ctrl_valid(&mut self) -> MPC_PLL_CTRL_VALID_W<7> {
        MPC_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn interconnect_sel_cka(&mut self) -> INTERCONNECT_SEL_CKA_W<8> {
        INTERCONNECT_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn interconnect_force_cka(&mut self) -> INTERCONNECT_FORCE_CKA_W<9> {
        INTERCONNECT_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn interconnect_force_ckb(&mut self) -> INTERCONNECT_FORCE_CKB_W<10> {
        INTERCONNECT_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn interconnect_subsys_clkena(&mut self) -> INTERCONNECT_SUBSYS_CLKENA_W<11> {
        INTERCONNECT_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn interconnect_pll_ctrl_valid(&mut self) -> INTERCONNECT_PLL_CTRL_VALID_W<15> {
        INTERCONNECT_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_sel_cka(&mut self) -> C2C_SEL_CKA_W<16> {
        C2C_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_force_cka(&mut self) -> C2C_FORCE_CKA_W<17> {
        C2C_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_force_ckb(&mut self) -> C2C_FORCE_CKB_W<18> {
        C2C_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_subsys_clkena(&mut self) -> C2C_SUBSYS_CLKENA_W<19> {
        C2C_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_pll_ctrl_valid(&mut self) -> C2C_PLL_CTRL_VALID_W<23> {
        C2C_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn corehw_sel_cka(&mut self) -> COREHW_SEL_CKA_W<24> {
        COREHW_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn corehw_force_cka(&mut self) -> COREHW_FORCE_CKA_W<25> {
        COREHW_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn corehw_force_ckb(&mut self) -> COREHW_FORCE_CKB_W<26> {
        COREHW_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn corehw_subsys_clkena(&mut self) -> COREHW_SUBSYS_CLKENA_W<27> {
        COREHW_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn corehw_pll_ctrl_valid(&mut self) -> COREHW_PLL_CTRL_VALID_W<31> {
        COREHW_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition for MPC, Interconnect, C2C and CoreHW subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl2](index.html) module"]
pub struct CLK_CTRL2_SPEC;
impl crate::RegisterSpec for CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl2::R](R) reader structure"]
impl crate::Readable for CLK_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl2::W](W) writer structure"]
impl crate::Writable for CLK_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CTRL2 to value 0"]
impl crate::Resettable for CLK_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
