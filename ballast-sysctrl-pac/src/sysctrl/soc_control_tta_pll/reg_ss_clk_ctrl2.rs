#[doc = "Register `REG_SS_CLK_CTRL2` reader"]
pub struct R(crate::R<REG_SS_CLK_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SS_CLK_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SS_CLK_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SS_CLK_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_SS_CLK_CTRL2` writer"]
pub struct W(crate::W<REG_SS_CLK_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SS_CLK_CTRL2_SPEC>;
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
impl From<crate::W<REG_SS_CLK_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SS_CLK_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mpc_sel_cka` reader - Select CKA"]
pub struct MPC_SEL_CKA_R(crate::FieldReader<bool, bool>);
impl MPC_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPC_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPC_SEL_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mpc_sel_cka` writer - Select CKA"]
pub struct MPC_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_SEL_CKA_W<'a> {
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
#[doc = "Field `mpc_force_cka` reader - Force CKA"]
pub struct MPC_FORCE_CKA_R(crate::FieldReader<bool, bool>);
impl MPC_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPC_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPC_FORCE_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mpc_force_cka` writer - Force CKA"]
pub struct MPC_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_FORCE_CKA_W<'a> {
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
#[doc = "Field `mpc_force_ckb` reader - Force CKB"]
pub struct MPC_FORCE_CKB_R(crate::FieldReader<bool, bool>);
impl MPC_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPC_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPC_FORCE_CKB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mpc_force_ckb` writer - Force CKB"]
pub struct MPC_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_FORCE_CKB_W<'a> {
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
#[doc = "Field `mpc_subsys_clkena` reader - Subsystem clock enable"]
pub struct MPC_SUBSYS_CLKENA_R(crate::FieldReader<bool, bool>);
impl MPC_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPC_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPC_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mpc_subsys_clkena` writer - Subsystem clock enable"]
pub struct MPC_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_SUBSYS_CLKENA_W<'a> {
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
#[doc = "Field `mpc_pll_ctrl_valid` reader - PLL Control valid"]
pub struct MPC_PLL_CTRL_VALID_R(crate::FieldReader<bool, bool>);
impl MPC_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPC_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPC_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mpc_pll_ctrl_valid` writer - PLL Control valid"]
pub struct MPC_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_PLL_CTRL_VALID_W<'a> {
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
#[doc = "Field `interconnect_sel_cka` reader - Select CKA"]
pub struct INTERCONNECT_SEL_CKA_R(crate::FieldReader<bool, bool>);
impl INTERCONNECT_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERCONNECT_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCONNECT_SEL_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interconnect_sel_cka` writer - Select CKA"]
pub struct INTERCONNECT_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCONNECT_SEL_CKA_W<'a> {
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
#[doc = "Field `interconnect_force_cka` reader - Force CKA"]
pub struct INTERCONNECT_FORCE_CKA_R(crate::FieldReader<bool, bool>);
impl INTERCONNECT_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERCONNECT_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCONNECT_FORCE_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interconnect_force_cka` writer - Force CKA"]
pub struct INTERCONNECT_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCONNECT_FORCE_CKA_W<'a> {
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
#[doc = "Field `interconnect_force_ckb` reader - Force CKB"]
pub struct INTERCONNECT_FORCE_CKB_R(crate::FieldReader<bool, bool>);
impl INTERCONNECT_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERCONNECT_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCONNECT_FORCE_CKB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interconnect_force_ckb` writer - Force CKB"]
pub struct INTERCONNECT_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCONNECT_FORCE_CKB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `interconnect_subsys_clkena` reader - Subsystem clock enable"]
pub struct INTERCONNECT_SUBSYS_CLKENA_R(crate::FieldReader<bool, bool>);
impl INTERCONNECT_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERCONNECT_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCONNECT_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interconnect_subsys_clkena` writer - Subsystem clock enable"]
pub struct INTERCONNECT_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCONNECT_SUBSYS_CLKENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `interconnect_pll_ctrl_valid` reader - PLL Control valid"]
pub struct INTERCONNECT_PLL_CTRL_VALID_R(crate::FieldReader<bool, bool>);
impl INTERCONNECT_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERCONNECT_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCONNECT_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interconnect_pll_ctrl_valid` writer - PLL Control valid"]
pub struct INTERCONNECT_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCONNECT_PLL_CTRL_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `c2c_sel_cka` reader - Select CKA"]
pub struct C2C_SEL_CKA_R(crate::FieldReader<bool, bool>);
impl C2C_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C2C_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_SEL_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c2c_sel_cka` writer - Select CKA"]
pub struct C2C_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_SEL_CKA_W<'a> {
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
#[doc = "Field `c2c_force_cka` reader - Force CKA"]
pub struct C2C_FORCE_CKA_R(crate::FieldReader<bool, bool>);
impl C2C_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C2C_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_FORCE_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c2c_force_cka` writer - Force CKA"]
pub struct C2C_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_FORCE_CKA_W<'a> {
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
#[doc = "Field `c2c_force_ckb` reader - Force CKB"]
pub struct C2C_FORCE_CKB_R(crate::FieldReader<bool, bool>);
impl C2C_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C2C_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_FORCE_CKB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c2c_force_ckb` writer - Force CKB"]
pub struct C2C_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_FORCE_CKB_W<'a> {
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
#[doc = "Field `c2c_subsys_clkena` reader - Subsystem clock enable"]
pub struct C2C_SUBSYS_CLKENA_R(crate::FieldReader<bool, bool>);
impl C2C_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C2C_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c2c_subsys_clkena` writer - Subsystem clock enable"]
pub struct C2C_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_SUBSYS_CLKENA_W<'a> {
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
#[doc = "Field `c2c_pll_ctrl_valid` reader - PLL Control valid"]
pub struct C2C_PLL_CTRL_VALID_R(crate::FieldReader<bool, bool>);
impl C2C_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C2C_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c2c_pll_ctrl_valid` writer - PLL Control valid"]
pub struct C2C_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_PLL_CTRL_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `corehw_sel_cka` reader - Select CKA"]
pub struct COREHW_SEL_CKA_R(crate::FieldReader<bool, bool>);
impl COREHW_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHW_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHW_SEL_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corehw_sel_cka` writer - Select CKA"]
pub struct COREHW_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHW_SEL_CKA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `corehw_force_cka` reader - Force CKA"]
pub struct COREHW_FORCE_CKA_R(crate::FieldReader<bool, bool>);
impl COREHW_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHW_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHW_FORCE_CKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corehw_force_cka` writer - Force CKA"]
pub struct COREHW_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHW_FORCE_CKA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `corehw_force_ckb` reader - Force CKB"]
pub struct COREHW_FORCE_CKB_R(crate::FieldReader<bool, bool>);
impl COREHW_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHW_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHW_FORCE_CKB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corehw_force_ckb` writer - Force CKB"]
pub struct COREHW_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHW_FORCE_CKB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `corehw_subsys_clkena` reader - Subsystem clock enable"]
pub struct COREHW_SUBSYS_CLKENA_R(crate::FieldReader<bool, bool>);
impl COREHW_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHW_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHW_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corehw_subsys_clkena` writer - Subsystem clock enable"]
pub struct COREHW_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHW_SUBSYS_CLKENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `corehw_pll_ctrl_valid` reader - PLL Control valid"]
pub struct COREHW_PLL_CTRL_VALID_R(crate::FieldReader<bool, bool>);
impl COREHW_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHW_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHW_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corehw_pll_ctrl_valid` writer - PLL Control valid"]
pub struct COREHW_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHW_PLL_CTRL_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
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
    pub fn mpc_sel_cka(&mut self) -> MPC_SEL_CKA_W {
        MPC_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn mpc_force_cka(&mut self) -> MPC_FORCE_CKA_W {
        MPC_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn mpc_force_ckb(&mut self) -> MPC_FORCE_CKB_W {
        MPC_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn mpc_subsys_clkena(&mut self) -> MPC_SUBSYS_CLKENA_W {
        MPC_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn mpc_pll_ctrl_valid(&mut self) -> MPC_PLL_CTRL_VALID_W {
        MPC_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    pub fn interconnect_sel_cka(&mut self) -> INTERCONNECT_SEL_CKA_W {
        INTERCONNECT_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    pub fn interconnect_force_cka(&mut self) -> INTERCONNECT_FORCE_CKA_W {
        INTERCONNECT_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    pub fn interconnect_force_ckb(&mut self) -> INTERCONNECT_FORCE_CKB_W {
        INTERCONNECT_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    pub fn interconnect_subsys_clkena(&mut self) -> INTERCONNECT_SUBSYS_CLKENA_W {
        INTERCONNECT_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    pub fn interconnect_pll_ctrl_valid(&mut self) -> INTERCONNECT_PLL_CTRL_VALID_W {
        INTERCONNECT_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    pub fn c2c_sel_cka(&mut self) -> C2C_SEL_CKA_W {
        C2C_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    pub fn c2c_force_cka(&mut self) -> C2C_FORCE_CKA_W {
        C2C_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    pub fn c2c_force_ckb(&mut self) -> C2C_FORCE_CKB_W {
        C2C_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    pub fn c2c_subsys_clkena(&mut self) -> C2C_SUBSYS_CLKENA_W {
        C2C_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    pub fn c2c_pll_ctrl_valid(&mut self) -> C2C_PLL_CTRL_VALID_W {
        C2C_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    pub fn corehw_sel_cka(&mut self) -> COREHW_SEL_CKA_W {
        COREHW_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    pub fn corehw_force_cka(&mut self) -> COREHW_FORCE_CKA_W {
        COREHW_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    pub fn corehw_force_ckb(&mut self) -> COREHW_FORCE_CKB_W {
        COREHW_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    pub fn corehw_subsys_clkena(&mut self) -> COREHW_SUBSYS_CLKENA_W {
        COREHW_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    pub fn corehw_pll_ctrl_valid(&mut self) -> COREHW_PLL_CTRL_VALID_W {
        COREHW_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition for MPC, Interconnect, C2C and CoreHW subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_ss_clk_ctrl2](index.html) module"]
pub struct REG_SS_CLK_CTRL2_SPEC;
impl crate::RegisterSpec for REG_SS_CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_ss_clk_ctrl2::R](R) reader structure"]
impl crate::Readable for REG_SS_CLK_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_ss_clk_ctrl2::W](W) writer structure"]
impl crate::Writable for REG_SS_CLK_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_SS_CLK_CTRL2 to value 0"]
impl crate::Resettable for REG_SS_CLK_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
