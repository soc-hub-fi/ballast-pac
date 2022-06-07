#[doc = "Register `D_DP_BS_MUL_CFG` reader"]
pub struct R(crate::R<D_DP_BS_MUL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DP_BS_MUL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DP_BS_MUL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DP_BS_MUL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DP_BS_MUL_CFG` writer"]
pub struct W(crate::W<D_DP_BS_MUL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DP_BS_MUL_CFG_SPEC>;
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
impl From<crate::W<D_DP_BS_MUL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DP_BS_MUL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_MUL_SRC_A {
    #[doc = "0: `0`"]
    REG = 0,
    #[doc = "1: `1`"]
    MEM = 1,
}
impl From<BS_MUL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: BS_MUL_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS_MUL_SRC` reader - "]
pub struct BS_MUL_SRC_R(crate::FieldReader<bool>);
impl BS_MUL_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BS_MUL_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_MUL_SRC_A {
        match self.bits {
            false => BS_MUL_SRC_A::REG,
            true => BS_MUL_SRC_A::MEM,
        }
    }
    #[doc = "Checks if the value of the field is `REG`"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        **self == BS_MUL_SRC_A::REG
    }
    #[doc = "Checks if the value of the field is `MEM`"]
    #[inline(always)]
    pub fn is_mem(&self) -> bool {
        **self == BS_MUL_SRC_A::MEM
    }
}
impl core::ops::Deref for BS_MUL_SRC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS_MUL_SRC` writer - "]
pub struct BS_MUL_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BS_MUL_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BS_MUL_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reg(self) -> &'a mut W {
        self.variant(BS_MUL_SRC_A::REG)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mem(self) -> &'a mut W {
        self.variant(BS_MUL_SRC_A::MEM)
    }
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
#[doc = "Field `BS_MUL_SHIFT_VALUE` reader - "]
pub struct BS_MUL_SHIFT_VALUE_R(crate::FieldReader<u8>);
impl BS_MUL_SHIFT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BS_MUL_SHIFT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS_MUL_SHIFT_VALUE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS_MUL_SHIFT_VALUE` writer - "]
pub struct BS_MUL_SHIFT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BS_MUL_SHIFT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bs_mul_src(&self) -> BS_MUL_SRC_R {
        BS_MUL_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bs_mul_shift_value(&self) -> BS_MUL_SHIFT_VALUE_R {
        BS_MUL_SHIFT_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bs_mul_src(&mut self) -> BS_MUL_SRC_W {
        BS_MUL_SRC_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bs_mul_shift_value(&mut self) -> BS_MUL_SHIFT_VALUE_W {
        BS_MUL_SHIFT_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source type and shifter value of BS MUL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dp_bs_mul_cfg](index.html) module"]
pub struct D_DP_BS_MUL_CFG_SPEC;
impl crate::RegisterSpec for D_DP_BS_MUL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dp_bs_mul_cfg::R](R) reader structure"]
impl crate::Readable for D_DP_BS_MUL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dp_bs_mul_cfg::W](W) writer structure"]
impl crate::Writable for D_DP_BS_MUL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DP_BS_MUL_CFG to value 0"]
impl crate::Resettable for D_DP_BS_MUL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
