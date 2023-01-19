#[doc = "Register `S_LUT_CFG` reader"]
pub struct R(crate::R<S_LUT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_CFG` writer"]
pub struct W(crate::W<S_LUT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_CFG_SPEC>;
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
impl From<crate::W<S_LUT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LE_FUNCTION` reader - "]
pub type LUT_LE_FUNCTION_R = crate::BitReader<LUT_LE_FUNCTION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT_LE_FUNCTION_A {
    #[doc = "0: `0`"]
    EXPONENT = 0,
    #[doc = "1: `1`"]
    LINEAR = 1,
}
impl From<LUT_LE_FUNCTION_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_LE_FUNCTION_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT_LE_FUNCTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_LE_FUNCTION_A {
        match self.bits {
            false => LUT_LE_FUNCTION_A::EXPONENT,
            true => LUT_LE_FUNCTION_A::LINEAR,
        }
    }
    #[doc = "Checks if the value of the field is `EXPONENT`"]
    #[inline(always)]
    pub fn is_exponent(&self) -> bool {
        *self == LUT_LE_FUNCTION_A::EXPONENT
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == LUT_LE_FUNCTION_A::LINEAR
    }
}
#[doc = "Field `LUT_LE_FUNCTION` writer - "]
pub type LUT_LE_FUNCTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, S_LUT_CFG_SPEC, LUT_LE_FUNCTION_A, O>;
impl<'a, const O: u8> LUT_LE_FUNCTION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn exponent(self) -> &'a mut W {
        self.variant(LUT_LE_FUNCTION_A::EXPONENT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut W {
        self.variant(LUT_LE_FUNCTION_A::LINEAR)
    }
}
#[doc = "Field `LUT_UFLOW_PRIORITY` reader - "]
pub type LUT_UFLOW_PRIORITY_R = crate::BitReader<LUT_UFLOW_PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT_UFLOW_PRIORITY_A {
    #[doc = "1: `1`"]
    LO = 1,
    #[doc = "0: `0`"]
    LE = 0,
}
impl From<LUT_UFLOW_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_UFLOW_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT_UFLOW_PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_UFLOW_PRIORITY_A {
        match self.bits {
            true => LUT_UFLOW_PRIORITY_A::LO,
            false => LUT_UFLOW_PRIORITY_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == LUT_UFLOW_PRIORITY_A::LO
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == LUT_UFLOW_PRIORITY_A::LE
    }
}
#[doc = "Field `LUT_UFLOW_PRIORITY` writer - "]
pub type LUT_UFLOW_PRIORITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, S_LUT_CFG_SPEC, LUT_UFLOW_PRIORITY_A, O>;
impl<'a, const O: u8> LUT_UFLOW_PRIORITY_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(LUT_UFLOW_PRIORITY_A::LO)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(LUT_UFLOW_PRIORITY_A::LE)
    }
}
#[doc = "Field `LUT_OFLOW_PRIORITY` reader - "]
pub type LUT_OFLOW_PRIORITY_R = crate::BitReader<LUT_OFLOW_PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT_OFLOW_PRIORITY_A {
    #[doc = "1: `1`"]
    LO = 1,
    #[doc = "0: `0`"]
    LE = 0,
}
impl From<LUT_OFLOW_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_OFLOW_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT_OFLOW_PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_OFLOW_PRIORITY_A {
        match self.bits {
            true => LUT_OFLOW_PRIORITY_A::LO,
            false => LUT_OFLOW_PRIORITY_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == LUT_OFLOW_PRIORITY_A::LO
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == LUT_OFLOW_PRIORITY_A::LE
    }
}
#[doc = "Field `LUT_OFLOW_PRIORITY` writer - "]
pub type LUT_OFLOW_PRIORITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, S_LUT_CFG_SPEC, LUT_OFLOW_PRIORITY_A, O>;
impl<'a, const O: u8> LUT_OFLOW_PRIORITY_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(LUT_OFLOW_PRIORITY_A::LO)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(LUT_OFLOW_PRIORITY_A::LE)
    }
}
#[doc = "Field `LUT_HYBRID_PRIORITY` reader - "]
pub type LUT_HYBRID_PRIORITY_R = crate::BitReader<LUT_HYBRID_PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT_HYBRID_PRIORITY_A {
    #[doc = "1: `1`"]
    LO = 1,
    #[doc = "0: `0`"]
    LE = 0,
}
impl From<LUT_HYBRID_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_HYBRID_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT_HYBRID_PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_HYBRID_PRIORITY_A {
        match self.bits {
            true => LUT_HYBRID_PRIORITY_A::LO,
            false => LUT_HYBRID_PRIORITY_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == LUT_HYBRID_PRIORITY_A::LO
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == LUT_HYBRID_PRIORITY_A::LE
    }
}
#[doc = "Field `LUT_HYBRID_PRIORITY` writer - "]
pub type LUT_HYBRID_PRIORITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, S_LUT_CFG_SPEC, LUT_HYBRID_PRIORITY_A, O>;
impl<'a, const O: u8> LUT_HYBRID_PRIORITY_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(LUT_HYBRID_PRIORITY_A::LO)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(LUT_HYBRID_PRIORITY_A::LE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lut_le_function(&self) -> LUT_LE_FUNCTION_R {
        LUT_LE_FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lut_uflow_priority(&self) -> LUT_UFLOW_PRIORITY_R {
        LUT_UFLOW_PRIORITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lut_oflow_priority(&self) -> LUT_OFLOW_PRIORITY_R {
        LUT_OFLOW_PRIORITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lut_hybrid_priority(&self) -> LUT_HYBRID_PRIORITY_R {
        LUT_HYBRID_PRIORITY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lut_le_function(&mut self) -> LUT_LE_FUNCTION_W<0> {
        LUT_LE_FUNCTION_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lut_uflow_priority(&mut self) -> LUT_UFLOW_PRIORITY_W<4> {
        LUT_UFLOW_PRIORITY_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lut_oflow_priority(&mut self) -> LUT_OFLOW_PRIORITY_W<5> {
        LUT_OFLOW_PRIORITY_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lut_hybrid_priority(&mut self) -> LUT_HYBRID_PRIORITY_W<6> {
        LUT_HYBRID_PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUTs type: exponent or linear. And the selection between LE and LO tables.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_cfg](index.html) module"]
pub struct S_LUT_CFG_SPEC;
impl crate::RegisterSpec for S_LUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_cfg::R](R) reader structure"]
impl crate::Readable for S_LUT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_cfg::W](W) writer structure"]
impl crate::Writable for S_LUT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_CFG to value 0"]
impl crate::Resettable for S_LUT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
