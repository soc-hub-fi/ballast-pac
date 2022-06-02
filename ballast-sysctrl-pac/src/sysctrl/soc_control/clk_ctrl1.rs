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
pub struct TTA_SEL_CKA_R(crate::FieldReader<bool>);
impl TTA_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTA_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_SEL_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tta_sel_cka` writer - Select CKA"]
pub struct TTA_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_SEL_CKA_W<'a> {
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
#[doc = "Field `tta_force_cka` reader - Force CKA"]
pub struct TTA_FORCE_CKA_R(crate::FieldReader<bool>);
impl TTA_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTA_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_FORCE_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tta_force_cka` writer - Force CKA"]
pub struct TTA_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_FORCE_CKA_W<'a> {
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
#[doc = "Field `tta_force_ckb` reader - Force CKB"]
pub struct TTA_FORCE_CKB_R(crate::FieldReader<bool>);
impl TTA_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTA_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_FORCE_CKB_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tta_force_ckb` writer - Force CKB"]
pub struct TTA_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_FORCE_CKB_W<'a> {
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
#[doc = "Field `tta_subsys_clkena` reader - Subsystem clock enable"]
pub struct TTA_SUBSYS_CLKENA_R(crate::FieldReader<bool>);
impl TTA_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTA_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tta_subsys_clkena` writer - Subsystem clock enable"]
pub struct TTA_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_SUBSYS_CLKENA_W<'a> {
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
#[doc = "Field `tta_pll_ctrl_valid` reader - PLL Control valid"]
pub struct TTA_PLL_CTRL_VALID_R(crate::FieldReader<bool>);
impl TTA_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTA_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tta_pll_ctrl_valid` writer - PLL Control valid"]
pub struct TTA_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_PLL_CTRL_VALID_W<'a> {
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
#[doc = "Field `ethernet_sel_cka` reader - Select CKA"]
pub struct ETHERNET_SEL_CKA_R(crate::FieldReader<bool>);
impl ETHERNET_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHERNET_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_SEL_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ethernet_sel_cka` writer - Select CKA"]
pub struct ETHERNET_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_SEL_CKA_W<'a> {
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
#[doc = "Field `ethernet_force_cka` reader - Force CKA"]
pub struct ETHERNET_FORCE_CKA_R(crate::FieldReader<bool>);
impl ETHERNET_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHERNET_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_FORCE_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ethernet_force_cka` writer - Force CKA"]
pub struct ETHERNET_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_FORCE_CKA_W<'a> {
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
#[doc = "Field `ethernet_force_ckb` reader - Force CKB"]
pub struct ETHERNET_FORCE_CKB_R(crate::FieldReader<bool>);
impl ETHERNET_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHERNET_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_FORCE_CKB_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ethernet_force_ckb` writer - Force CKB"]
pub struct ETHERNET_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_FORCE_CKB_W<'a> {
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
#[doc = "Field `ethernet_subsys_clkena` reader - Subsystem clock enable"]
pub struct ETHERNET_SUBSYS_CLKENA_R(crate::FieldReader<bool>);
impl ETHERNET_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHERNET_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ethernet_subsys_clkena` writer - Subsystem clock enable"]
pub struct ETHERNET_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_SUBSYS_CLKENA_W<'a> {
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
#[doc = "Field `ethernet_pll_ctrl_valid` reader - PLL Control valid"]
pub struct ETHERNET_PLL_CTRL_VALID_R(crate::FieldReader<bool>);
impl ETHERNET_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHERNET_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ethernet_pll_ctrl_valid` writer - PLL Control valid"]
pub struct ETHERNET_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_PLL_CTRL_VALID_W<'a> {
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
#[doc = "Field `AI_sel_cka` reader - Select CKA"]
pub struct AI_SEL_CKA_R(crate::FieldReader<bool>);
impl AI_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_SEL_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI_sel_cka` writer - Select CKA"]
pub struct AI_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_SEL_CKA_W<'a> {
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
#[doc = "Field `AI_force_cka` reader - Force CKA"]
pub struct AI_FORCE_CKA_R(crate::FieldReader<bool>);
impl AI_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_FORCE_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI_force_cka` writer - Force CKA"]
pub struct AI_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_FORCE_CKA_W<'a> {
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
#[doc = "Field `AI_force_ckb` reader - Force CKB"]
pub struct AI_FORCE_CKB_R(crate::FieldReader<bool>);
impl AI_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_FORCE_CKB_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI_force_ckb` writer - Force CKB"]
pub struct AI_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_FORCE_CKB_W<'a> {
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
#[doc = "Field `AI_subsys_clkena` reader - Subsystem clock enable"]
pub struct AI_SUBSYS_CLKENA_R(crate::FieldReader<bool>);
impl AI_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI_subsys_clkena` writer - Subsystem clock enable"]
pub struct AI_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_SUBSYS_CLKENA_W<'a> {
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
#[doc = "Field `AI_pll_ctrl_valid` reader - PLL Control valid"]
pub struct AI_PLL_CTRL_VALID_R(crate::FieldReader<bool>);
impl AI_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI_pll_ctrl_valid` writer - PLL Control valid"]
pub struct AI_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_PLL_CTRL_VALID_W<'a> {
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
#[doc = "Field `hpc_sel_cka` reader - Select CKA"]
pub struct HPC_SEL_CKA_R(crate::FieldReader<bool>);
impl HPC_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPC_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_SEL_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hpc_sel_cka` writer - Select CKA"]
pub struct HPC_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_SEL_CKA_W<'a> {
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
#[doc = "Field `hpc_force_cka` reader - Force CKA"]
pub struct HPC_FORCE_CKA_R(crate::FieldReader<bool>);
impl HPC_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPC_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_FORCE_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hpc_force_cka` writer - Force CKA"]
pub struct HPC_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_FORCE_CKA_W<'a> {
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
#[doc = "Field `hpc_force_ckb` reader - Force CKB"]
pub struct HPC_FORCE_CKB_R(crate::FieldReader<bool>);
impl HPC_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPC_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_FORCE_CKB_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hpc_force_ckb` writer - Force CKB"]
pub struct HPC_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_FORCE_CKB_W<'a> {
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
#[doc = "Field `hpc_subsys_clkena` reader - Subsystem clock enable"]
pub struct HPC_SUBSYS_CLKENA_R(crate::FieldReader<bool>);
impl HPC_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPC_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hpc_subsys_clkena` writer - Subsystem clock enable"]
pub struct HPC_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_SUBSYS_CLKENA_W<'a> {
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
#[doc = "Field `hpc_pll_ctrl_valid` reader - PLL Control valid"]
pub struct HPC_PLL_CTRL_VALID_R(crate::FieldReader<bool>);
impl HPC_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPC_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hpc_pll_ctrl_valid` writer - PLL Control valid"]
pub struct HPC_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_PLL_CTRL_VALID_W<'a> {
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
    pub fn tta_sel_cka(&mut self) -> TTA_SEL_CKA_W {
        TTA_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn tta_force_cka(&mut self) -> TTA_FORCE_CKA_W {
        TTA_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn tta_force_ckb(&mut self) -> TTA_FORCE_CKB_W {
        TTA_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn tta_subsys_clkena(&mut self) -> TTA_SUBSYS_CLKENA_W {
        TTA_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn tta_pll_ctrl_valid(&mut self) -> TTA_PLL_CTRL_VALID_W {
        TTA_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    pub fn ethernet_sel_cka(&mut self) -> ETHERNET_SEL_CKA_W {
        ETHERNET_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    pub fn ethernet_force_cka(&mut self) -> ETHERNET_FORCE_CKA_W {
        ETHERNET_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    pub fn ethernet_force_ckb(&mut self) -> ETHERNET_FORCE_CKB_W {
        ETHERNET_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    pub fn ethernet_subsys_clkena(&mut self) -> ETHERNET_SUBSYS_CLKENA_W {
        ETHERNET_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    pub fn ethernet_pll_ctrl_valid(&mut self) -> ETHERNET_PLL_CTRL_VALID_W {
        ETHERNET_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    pub fn ai_sel_cka(&mut self) -> AI_SEL_CKA_W {
        AI_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    pub fn ai_force_cka(&mut self) -> AI_FORCE_CKA_W {
        AI_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    pub fn ai_force_ckb(&mut self) -> AI_FORCE_CKB_W {
        AI_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    pub fn ai_subsys_clkena(&mut self) -> AI_SUBSYS_CLKENA_W {
        AI_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    pub fn ai_pll_ctrl_valid(&mut self) -> AI_PLL_CTRL_VALID_W {
        AI_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    pub fn hpc_sel_cka(&mut self) -> HPC_SEL_CKA_W {
        HPC_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    pub fn hpc_force_cka(&mut self) -> HPC_FORCE_CKA_W {
        HPC_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    pub fn hpc_force_ckb(&mut self) -> HPC_FORCE_CKB_W {
        HPC_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    pub fn hpc_subsys_clkena(&mut self) -> HPC_SUBSYS_CLKENA_W {
        HPC_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    pub fn hpc_pll_ctrl_valid(&mut self) -> HPC_PLL_CTRL_VALID_W {
        HPC_PLL_CTRL_VALID_W { w: self }
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
}
#[doc = "`reset()` method sets CLK_CTRL1 to value 0"]
impl crate::Resettable for CLK_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
