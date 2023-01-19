#[doc = "Register `D_DP_EW_ALU_CFG` reader"]
pub struct R(crate::R<D_DP_EW_ALU_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_EW_ALU_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_EW_ALU_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_EW_ALU_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DP_EW_ALU_CFG` writer"]
pub struct W(crate::W<D_DP_EW_ALU_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_EW_ALU_CFG_SPEC>;
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
impl From<crate::W<D_DP_EW_ALU_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_EW_ALU_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EW_ALU_SRC` reader - "]
pub type EW_ALU_SRC_R = crate::BitReader<EW_ALU_SRC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW_ALU_SRC_A {
    #[doc = "0: `0`"]
    REG = 0,
    #[doc = "1: `1`"]
    MEM = 1,
}
impl From<EW_ALU_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: EW_ALU_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl EW_ALU_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_ALU_SRC_A {
        match self.bits {
            false => EW_ALU_SRC_A::REG,
            true => EW_ALU_SRC_A::MEM,
        }
    }
    #[doc = "Checks if the value of the field is `REG`"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        *self == EW_ALU_SRC_A::REG
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        *self == EW_ALU_SRC_A::MEM
    }
}
#[doc = "Field `EW_ALU_SRC` writer - "]
pub type EW_ALU_SRC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_EW_ALU_CFG_SPEC, EW_ALU_SRC_A, O>;
impl<'a, const O: u8> EW_ALU_SRC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reg(self) -> &'a mut W {
        self.variant(EW_ALU_SRC_A::REG)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mem(self) -> &'a mut W {
        self.variant(EW_ALU_SRC_A::MEM)
    }
}
#[doc = "Field `EW_ALU_CVT_BYPASS` reader - "]
pub type EW_ALU_CVT_BYPASS_R = crate::BitReader<EW_ALU_CVT_BYPASS_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW_ALU_CVT_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<EW_ALU_CVT_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: EW_ALU_CVT_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl EW_ALU_CVT_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_ALU_CVT_BYPASS_A {
        match self.bits {
            true => EW_ALU_CVT_BYPASS_A::YES,
            false => EW_ALU_CVT_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == EW_ALU_CVT_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == EW_ALU_CVT_BYPASS_A::NO
    }
}
#[doc = "Field `EW_ALU_CVT_BYPASS` writer - "]
pub type EW_ALU_CVT_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_EW_ALU_CFG_SPEC, EW_ALU_CVT_BYPASS_A, O>;
impl<'a, const O: u8> EW_ALU_CVT_BYPASS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(EW_ALU_CVT_BYPASS_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(EW_ALU_CVT_BYPASS_A::NO)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ew_alu_src(&self) -> EW_ALU_SRC_R {
        EW_ALU_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ew_alu_cvt_bypass(&self) -> EW_ALU_CVT_BYPASS_R {
        EW_ALU_CVT_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ew_alu_src(&mut self) -> EW_ALU_SRC_W<0> {
        EW_ALU_SRC_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ew_alu_cvt_bypass(&mut self) -> EW_ALU_CVT_BYPASS_W<1> {
        EW_ALU_CVT_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source type and bypass control of EW ALU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_ew_alu_cfg](index.html) module"]
pub struct D_DP_EW_ALU_CFG_SPEC;
impl crate::RegisterSpec for D_DP_EW_ALU_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_ew_alu_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_EW_ALU_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_ew_alu_cfg::W](W) writer structure"]
impl crate::Writable for D_DP_EW_ALU_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DP_EW_ALU_CFG to value 0x02"]
impl crate::Resettable for D_DP_EW_ALU_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
