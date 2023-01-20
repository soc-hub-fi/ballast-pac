#[doc = "Register `D_DP_BS_CFG` reader"]
pub struct R(crate::R<D_DP_BS_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BS_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BS_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DP_BS_CFG` writer"]
pub struct W(crate::W<D_DP_BS_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_BS_CFG_SPEC>;
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
impl From<crate::W<D_DP_BS_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_BS_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS_BYPASS` reader - "]
pub type BS_BYPASS_R = crate::BitReader<BS_BYPASS_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BS_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_BYPASS_A {
        match self.bits {
            true => BS_BYPASS_A::YES,
            false => BS_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BS_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BS_BYPASS_A::NO
    }
}
#[doc = "Field `BS_BYPASS` writer - "]
pub type BS_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, D_DP_BS_CFG_SPEC, BS_BYPASS_A, O>;
impl<'a, const O: u8> BS_BYPASS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BS_BYPASS_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BS_BYPASS_A::NO)
    }
}
#[doc = "Field `BS_ALU_BYPASS` reader - "]
pub type BS_ALU_BYPASS_R = crate::BitReader<BS_ALU_BYPASS_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS_ALU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_ALU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_ALU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BS_ALU_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_ALU_BYPASS_A {
        match self.bits {
            true => BS_ALU_BYPASS_A::YES,
            false => BS_ALU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BS_ALU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BS_ALU_BYPASS_A::NO
    }
}
#[doc = "Field `BS_ALU_BYPASS` writer - "]
pub type BS_ALU_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_BS_CFG_SPEC, BS_ALU_BYPASS_A, O>;
impl<'a, const O: u8> BS_ALU_BYPASS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BS_ALU_BYPASS_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BS_ALU_BYPASS_A::NO)
    }
}
#[doc = "Field `BS_ALU_ALGO` reader - "]
pub type BS_ALU_ALGO_R = crate::FieldReader<u8, BS_ALU_ALGO_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BS_ALU_ALGO_A {
    #[doc = "0: `0`"]
    MAX = 0,
    #[doc = "1: `1`"]
    MIN = 1,
    #[doc = "2: `10`"]
    SUM = 2,
}
impl From<BS_ALU_ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: BS_ALU_ALGO_A) -> Self {
        variant as _
    }
}
impl BS_ALU_ALGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BS_ALU_ALGO_A> {
        match self.bits {
            0 => Some(BS_ALU_ALGO_A::MAX),
            1 => Some(BS_ALU_ALGO_A::MIN),
            2 => Some(BS_ALU_ALGO_A::SUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == BS_ALU_ALGO_A::MAX
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == BS_ALU_ALGO_A::MIN
    }
    #[doc = "Checks if the value of the field is `SUM`"]
    #[inline(always)]
    pub fn is_sum(&self) -> bool {
        *self == BS_ALU_ALGO_A::SUM
    }
}
#[doc = "Field `BS_ALU_ALGO` writer - "]
pub type BS_ALU_ALGO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DP_BS_CFG_SPEC, u8, BS_ALU_ALGO_A, 2, O>;
impl<'a, const O: u8> BS_ALU_ALGO_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(BS_ALU_ALGO_A::MAX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(BS_ALU_ALGO_A::MIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sum(self) -> &'a mut W {
        self.variant(BS_ALU_ALGO_A::SUM)
    }
}
#[doc = "Field `BS_MUL_BYPASS` reader - "]
pub type BS_MUL_BYPASS_R = crate::BitReader<BS_MUL_BYPASS_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS_MUL_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_MUL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_MUL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BS_MUL_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_MUL_BYPASS_A {
        match self.bits {
            true => BS_MUL_BYPASS_A::YES,
            false => BS_MUL_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BS_MUL_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BS_MUL_BYPASS_A::NO
    }
}
#[doc = "Field `BS_MUL_BYPASS` writer - "]
pub type BS_MUL_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_BS_CFG_SPEC, BS_MUL_BYPASS_A, O>;
impl<'a, const O: u8> BS_MUL_BYPASS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BS_MUL_BYPASS_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BS_MUL_BYPASS_A::NO)
    }
}
#[doc = "Field `BS_MUL_PRELU` reader - "]
pub type BS_MUL_PRELU_R = crate::BitReader<BS_MUL_PRELU_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS_MUL_PRELU_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_MUL_PRELU_A> for bool {
    #[inline(always)]
    fn from(variant: BS_MUL_PRELU_A) -> Self {
        variant as u8 != 0
    }
}
impl BS_MUL_PRELU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_MUL_PRELU_A {
        match self.bits {
            true => BS_MUL_PRELU_A::YES,
            false => BS_MUL_PRELU_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BS_MUL_PRELU_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BS_MUL_PRELU_A::NO
    }
}
#[doc = "Field `BS_MUL_PRELU` writer - "]
pub type BS_MUL_PRELU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_BS_CFG_SPEC, BS_MUL_PRELU_A, O>;
impl<'a, const O: u8> BS_MUL_PRELU_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BS_MUL_PRELU_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BS_MUL_PRELU_A::NO)
    }
}
#[doc = "Field `BS_RELU_BYPASS` reader - "]
pub type BS_RELU_BYPASS_R = crate::BitReader<BS_RELU_BYPASS_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS_RELU_BYPASS_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<BS_RELU_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_RELU_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BS_RELU_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_RELU_BYPASS_A {
        match self.bits {
            true => BS_RELU_BYPASS_A::YES,
            false => BS_RELU_BYPASS_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BS_RELU_BYPASS_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BS_RELU_BYPASS_A::NO
    }
}
#[doc = "Field `BS_RELU_BYPASS` writer - "]
pub type BS_RELU_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DP_BS_CFG_SPEC, BS_RELU_BYPASS_A, O>;
impl<'a, const O: u8> BS_RELU_BYPASS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BS_RELU_BYPASS_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BS_RELU_BYPASS_A::NO)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bs_bypass(&self) -> BS_BYPASS_R {
        BS_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bs_alu_bypass(&self) -> BS_ALU_BYPASS_R {
        BS_ALU_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bs_alu_algo(&self) -> BS_ALU_ALGO_R {
        BS_ALU_ALGO_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bs_mul_bypass(&self) -> BS_MUL_BYPASS_R {
        BS_MUL_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bs_mul_prelu(&self) -> BS_MUL_PRELU_R {
        BS_MUL_PRELU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bs_relu_bypass(&self) -> BS_RELU_BYPASS_R {
        BS_RELU_BYPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bs_bypass(&mut self) -> BS_BYPASS_W<0> {
        BS_BYPASS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs_alu_bypass(&mut self) -> BS_ALU_BYPASS_W<1> {
        BS_ALU_BYPASS_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn bs_alu_algo(&mut self) -> BS_ALU_ALGO_W<2> {
        BS_ALU_ALGO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bs_mul_bypass(&mut self) -> BS_MUL_BYPASS_W<4> {
        BS_MUL_BYPASS_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bs_mul_prelu(&mut self) -> BS_MUL_PRELU_W<5> {
        BS_MUL_PRELU_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bs_relu_bypass(&mut self) -> BS_RELU_BYPASS_W<6> {
        BS_RELU_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurations of BS module: bypass, algorithm, etc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bs_cfg](index.html) module"]
pub struct D_DP_BS_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bs_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BS_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_bs_cfg::W](W) writer structure"]
impl crate::Writable for D_DP_BS_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DP_BS_CFG to value 0x73"]
impl crate::Resettable for D_DP_BS_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x73;
}
