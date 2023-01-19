#[doc = "Register `D_MISC_CFG` reader"]
pub struct R(crate::R<D_MISC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_MISC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_MISC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_MISC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_MISC_CFG` writer"]
pub struct W(crate::W<D_MISC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_MISC_CFG_SPEC>;
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
impl From<crate::W<D_MISC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_MISC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV_MODE` reader - "]
pub type CONV_MODE_R = crate::BitReader<CONV_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONV_MODE_A {
    #[doc = "0: `0`"]
    DIRECT = 0,
    #[doc = "1: `1`"]
    WINOGRAD = 1,
}
impl From<CONV_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CONV_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CONV_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONV_MODE_A {
        match self.bits {
            false => CONV_MODE_A::DIRECT,
            true => CONV_MODE_A::WINOGRAD,
        }
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CONV_MODE_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `WINOGRAD`"]
    #[inline(always)]
    pub fn is_winograd(&self) -> bool {
        *self == CONV_MODE_A::WINOGRAD
    }
}
#[doc = "Field `CONV_MODE` writer - "]
pub type CONV_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, D_MISC_CFG_SPEC, CONV_MODE_A, O>;
impl<'a, const O: u8> CONV_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CONV_MODE_A::DIRECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn winograd(self) -> &'a mut W {
        self.variant(CONV_MODE_A::WINOGRAD)
    }
}
#[doc = "Field `IN_PRECISION` reader - "]
pub type IN_PRECISION_R = crate::FieldReader<u8, IN_PRECISION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN_PRECISION_A {
    #[doc = "0: `0`"]
    INT8 = 0,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "2: `10`"]
    FP16 = 2,
}
impl From<IN_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: IN_PRECISION_A) -> Self {
        variant as _
    }
}
impl IN_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IN_PRECISION_A> {
        match self.bits {
            0 => Some(IN_PRECISION_A::INT8),
            1 => Some(IN_PRECISION_A::INT16),
            2 => Some(IN_PRECISION_A::FP16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == IN_PRECISION_A::INT8
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == IN_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == IN_PRECISION_A::FP16
    }
}
#[doc = "Field `IN_PRECISION` writer - "]
pub type IN_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_MISC_CFG_SPEC, u8, IN_PRECISION_A, 2, O>;
impl<'a, const O: u8> IN_PRECISION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(IN_PRECISION_A::INT8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(IN_PRECISION_A::INT16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(IN_PRECISION_A::FP16)
    }
}
#[doc = "Field `PROC_PRECISION` reader - "]
pub type PROC_PRECISION_R = crate::FieldReader<u8, PROC_PRECISION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROC_PRECISION_A {
    #[doc = "2: `10`"]
    FP16 = 2,
    #[doc = "1: `1`"]
    INT16 = 1,
    #[doc = "0: `0`"]
    INT8 = 0,
}
impl From<PROC_PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: PROC_PRECISION_A) -> Self {
        variant as _
    }
}
impl PROC_PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROC_PRECISION_A> {
        match self.bits {
            2 => Some(PROC_PRECISION_A::FP16),
            1 => Some(PROC_PRECISION_A::INT16),
            0 => Some(PROC_PRECISION_A::INT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FP16`"]
    #[inline(always)]
    pub fn is_fp16(&self) -> bool {
        *self == PROC_PRECISION_A::FP16
    }
    #[doc = "Checks if the value of the field is `INT16`"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == PROC_PRECISION_A::INT16
    }
    #[doc = "Checks if the value of the field is `INT8`"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == PROC_PRECISION_A::INT8
    }
}
#[doc = "Field `PROC_PRECISION` writer - "]
pub type PROC_PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_MISC_CFG_SPEC, u8, PROC_PRECISION_A, 2, O>;
impl<'a, const O: u8> PROC_PRECISION_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fp16(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::FP16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::INT16)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut W {
        self.variant(PROC_PRECISION_A::INT8)
    }
}
#[doc = "Field `DATA_REUSE` reader - "]
pub type DATA_REUSE_R = crate::BitReader<DATA_REUSE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_REUSE_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<DATA_REUSE_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_REUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_REUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_REUSE_A {
        match self.bits {
            true => DATA_REUSE_A::ENABLE,
            false => DATA_REUSE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DATA_REUSE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DATA_REUSE_A::DISABLE
    }
}
#[doc = "Field `DATA_REUSE` writer - "]
pub type DATA_REUSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_MISC_CFG_SPEC, DATA_REUSE_A, O>;
impl<'a, const O: u8> DATA_REUSE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DATA_REUSE_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DATA_REUSE_A::DISABLE)
    }
}
#[doc = "Field `WEIGHT_REUSE` reader - "]
pub type WEIGHT_REUSE_R = crate::BitReader<WEIGHT_REUSE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WEIGHT_REUSE_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<WEIGHT_REUSE_A> for bool {
    #[inline(always)]
    fn from(variant: WEIGHT_REUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl WEIGHT_REUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEIGHT_REUSE_A {
        match self.bits {
            true => WEIGHT_REUSE_A::ENABLE,
            false => WEIGHT_REUSE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WEIGHT_REUSE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WEIGHT_REUSE_A::DISABLE
    }
}
#[doc = "Field `WEIGHT_REUSE` writer - "]
pub type WEIGHT_REUSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_MISC_CFG_SPEC, WEIGHT_REUSE_A, O>;
impl<'a, const O: u8> WEIGHT_REUSE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WEIGHT_REUSE_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WEIGHT_REUSE_A::DISABLE)
    }
}
#[doc = "Field `SKIP_DATA_RLS` reader - "]
pub type SKIP_DATA_RLS_R = crate::BitReader<SKIP_DATA_RLS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKIP_DATA_RLS_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<SKIP_DATA_RLS_A> for bool {
    #[inline(always)]
    fn from(variant: SKIP_DATA_RLS_A) -> Self {
        variant as u8 != 0
    }
}
impl SKIP_DATA_RLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIP_DATA_RLS_A {
        match self.bits {
            true => SKIP_DATA_RLS_A::ENABLE,
            false => SKIP_DATA_RLS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SKIP_DATA_RLS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SKIP_DATA_RLS_A::DISABLE
    }
}
#[doc = "Field `SKIP_DATA_RLS` writer - "]
pub type SKIP_DATA_RLS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_MISC_CFG_SPEC, SKIP_DATA_RLS_A, O>;
impl<'a, const O: u8> SKIP_DATA_RLS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKIP_DATA_RLS_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKIP_DATA_RLS_A::DISABLE)
    }
}
#[doc = "Field `SKIP_WEIGHT_RLS` reader - "]
pub type SKIP_WEIGHT_RLS_R = crate::BitReader<SKIP_WEIGHT_RLS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKIP_WEIGHT_RLS_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<SKIP_WEIGHT_RLS_A> for bool {
    #[inline(always)]
    fn from(variant: SKIP_WEIGHT_RLS_A) -> Self {
        variant as u8 != 0
    }
}
impl SKIP_WEIGHT_RLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIP_WEIGHT_RLS_A {
        match self.bits {
            true => SKIP_WEIGHT_RLS_A::ENABLE,
            false => SKIP_WEIGHT_RLS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SKIP_WEIGHT_RLS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SKIP_WEIGHT_RLS_A::DISABLE
    }
}
#[doc = "Field `SKIP_WEIGHT_RLS` writer - "]
pub type SKIP_WEIGHT_RLS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_MISC_CFG_SPEC, SKIP_WEIGHT_RLS_A, O>;
impl<'a, const O: u8> SKIP_WEIGHT_RLS_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKIP_WEIGHT_RLS_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKIP_WEIGHT_RLS_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn conv_mode(&self) -> CONV_MODE_R {
        CONV_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn in_precision(&self) -> IN_PRECISION_R {
        IN_PRECISION_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn proc_precision(&self) -> PROC_PRECISION_R {
        PROC_PRECISION_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn data_reuse(&self) -> DATA_REUSE_R {
        DATA_REUSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn weight_reuse(&self) -> WEIGHT_REUSE_R {
        WEIGHT_REUSE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn skip_data_rls(&self) -> SKIP_DATA_RLS_R {
        SKIP_DATA_RLS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn skip_weight_rls(&self) -> SKIP_WEIGHT_RLS_R {
        SKIP_WEIGHT_RLS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn conv_mode(&mut self) -> CONV_MODE_W<0> {
        CONV_MODE_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn in_precision(&mut self) -> IN_PRECISION_W<8> {
        IN_PRECISION_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn proc_precision(&mut self) -> PROC_PRECISION_W<12> {
        PROC_PRECISION_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn data_reuse(&mut self) -> DATA_REUSE_W<16> {
        DATA_REUSE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn weight_reuse(&mut self) -> WEIGHT_REUSE_W<20> {
        WEIGHT_REUSE_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn skip_data_rls(&mut self) -> SKIP_DATA_RLS_W<24> {
        SKIP_DATA_RLS_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn skip_weight_rls(&mut self) -> SKIP_WEIGHT_RLS_W<28> {
        SKIP_WEIGHT_RLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of operation: convolution mode, precision, weight reuse, data reuse.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_misc_cfg](index.html) module"]
pub struct D_MISC_CFG_SPEC;
impl crate::RegisterSpec for D_MISC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_misc_cfg::R](R) reader structure"]
impl crate::Readable for D_MISC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_misc_cfg::W](W) writer structure"]
impl crate::Writable for D_MISC_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_MISC_CFG to value 0"]
impl crate::Resettable for D_MISC_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
